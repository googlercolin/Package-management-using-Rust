use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

use crate::Packages;
use crate::packages::{Dependency, RelVersionedPackageNum};

use rpkg::debversion;

const KEYVAL_REGEX : &str = r"(?P<key>(\w|-)+): (?P<value>.+)";
const PKGNAME_AND_VERSION_REGEX : &str = r"(?P<pkg>(\w|\.|\+|-)+)( \((?P<op>(<|=|>)(<|=|>)?) (?P<ver>.*)\))?";

impl Packages {
    /// Loads packages and version numbers from a file, calling get_package_num_inserting on the package name
    /// and inserting the appropriate value into the installed_debvers map with the parsed version number.
    pub fn parse_installed(&mut self, filename: &str) {
        let kv_regexp = Regex::new(KEYVAL_REGEX).unwrap();
        if let Ok(lines) = read_lines(filename) {
            let mut current_package_num = 0;
            for line in lines {
                if let Ok(ip) = line {
                    // do something with ip
                    match kv_regexp.captures(&ip) {
                        None => (),
                        Some(caps) => {
                            let (key, value) = (caps.name("key").unwrap().as_str(),
                                                caps.name("value").unwrap().as_str());
                            if key == "Package" {
                                current_package_num =
                                    self.get_package_num_inserting(&value);
                            }

                            else if key == "Version" {
                                let debver =
                                    value.trim().parse::<debversion::DebianVersionNum>().unwrap();
                                self.installed_debvers.insert(current_package_num, debver);
                            }
                        }
                        }
                }
            }
        }
        println!("Packages installed: {}", self.installed_debvers.keys().len());
    }

    /// Loads packages, version numbers, dependencies, and md5sums from a file, calling get_package_num_inserting on the package name
    /// and inserting the appropriate values into the dependencies, md5sum, and available_debvers maps.
    pub fn parse_packages(&mut self, filename: &str) {
        let kv_regexp = Regex::new(KEYVAL_REGEX).unwrap();
        let pkgver_regexp = Regex::new(PKGNAME_AND_VERSION_REGEX).unwrap();

        if let Ok(lines) = read_lines(filename) {
            let mut current_package_num = 0;
            for line in lines {
                if let Ok(ip) = line {
                    // do more things with ip
                    match kv_regexp.captures(&ip) {
                        None => (),
                        Some(caps) => {
                            let (key, value) = (caps.name("key").unwrap().as_str(),
                                                caps.name("value").unwrap().as_str());

                            match key {
                                "Package" => {
                                    current_package_num =
                                        self.get_package_num_inserting(value);
                                    // println!("pkg: {} {}", value, current_package_num);
                                },
                                "Version" => {
                                    // println!("ver-1: {} {}", current_package_num, value);
                                    let debver =
                                        value.trim().parse::<debversion::DebianVersionNum>().unwrap();
                                    self.available_debvers.insert(current_package_num, debver);
                                    // println!("ver-2: {} {}", current_package_num, value);
                                },
                                "MD5sum" => {
                                    // println!("md5-1: {} {} {}", current_package_num, value, value.trim().to_string());
                                    self.md5sums.insert(current_package_num, value.trim().to_string());
                                    // println!("md5-2: {} {} {}", current_package_num, value, value.trim().to_string());
                                },
                                "Depends" => {
                                    let all_deps = value.to_string();
                                    let all_deps = all_deps.split(",");
                                    let all_deps: Vec<&str> = all_deps.collect();
                                    let mut vec_str_deps: Vec<Vec<&str>> = vec![];
                                    let mut final_deps: Vec<Dependency> = vec![];
                                    for a_deps in all_deps {
                                        let a_deps = a_deps.split("|");
                                        let a_deps: Vec<&str> = a_deps.collect();
                                        vec_str_deps.push(a_deps);
                                    }
                                    for vec_str_dep in vec_str_deps {
                                        let mut final_dep: Dependency = vec![];
                                        for str_dep in vec_str_dep {
                                            match pkgver_regexp.captures(str_dep) {
                                                None => (),
                                                Some(caps) => {
                                                    let pkg = caps.name("pkg").unwrap().as_str();
                                                    let package_num =
                                                        self.get_package_num_inserting(pkg);

                                                    let op = match caps.name("op") {
                                                        Some(x) =>
                                                            Some(x.as_str().parse::<debversion::VersionRelation>().unwrap()),
                                                        None => None,
                                                    };

                                                    let ver = match caps.name("ver") {
                                                        Some(x) => Some(x.as_str().to_string()),
                                                        None => None,
                                                    };

                                                    match op {
                                                        None => {
                                                            let dep = RelVersionedPackageNum {
                                                                package_num,
                                                                rel_version: None,
                                                            };
                                                            final_dep.push(dep);
                                                        }
                                                        Some(x) => {
                                                            let dep = RelVersionedPackageNum {
                                                                package_num,
                                                                rel_version: Some((x, ver.unwrap())),
                                                            };
                                                            final_dep.push(dep);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        final_deps.push(final_dep);
                                    }
                                    // println!("{:?}", final_deps);
                                    self.dependencies.insert(current_package_num, final_deps);
                                },
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        println!("Packages available: {}", self.available_debvers.keys().len());
    }
}


// standard template code downloaded from the Internet somewhere
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
