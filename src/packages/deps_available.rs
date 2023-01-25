use rpkg::debversion;
use crate::Packages;
use crate::packages::Dependency;

impl Packages {
    /// Gets the dependencies of package_name, and prints out whether they are satisfied
    /// (and by which library/version) or not.
    pub fn deps_available(&self, package_name: &str) {
        if !self.package_exists(package_name) {
            println!("no such package {}", package_name);
            return;
        }
        println!("Package {}:", package_name);
        // println!("+ {} satisfied by installed version {}", "dep", "459");
        // some sort of for loop...
        let package_num = self.get_package_num(package_name);
        if let Some(deps) = self.dependencies.get(package_num){
            for dep in deps {
                println!("- dependency {:?}", self.dep2str(dep));
                match self.dep_is_satisfied(dep) {
                    None => println!("-> not satisfied"),
                    Some(x) => {
                        if let Some(ver) =
                            self.installed_debvers.get(self.get_package_num(x)) {
                            println!("+ {} satisfied by installed version {}", x, ver);
                        };

                    }
                }
            }
        };
    }

    /// Returns Some(package) which satisfies dependency dd, or None if not satisfied.
    pub fn dep_is_satisfied(&self, dd:&Dependency) -> Option<&str> {
        // presumably you should loop on dd
        for dep in dd {
            match self.installed_debvers.get(&dep.package_num) {
                None => (),
                Some(v) => {
                    if let Some((op, ver)) = &dep.rel_version {
                        let ver = ver.parse::<debversion::DebianVersionNum>().unwrap();
                        if debversion::cmp_debversion_with_op(&op, &v, &ver) {
                            let package = self.get_package_name(dep.package_num);
                            return Some(package);
                        }
                    } else {
                        let package = self.get_package_name(dep.package_num);
                        return Some(package);
                    }
                    // let (op, iv) = (dep.rel_version.unwrap().0, dep.rel_version.unwrap().1);
                }
            }
        }
        return None;
    }

    /// Returns a Vec of packages which would satisfy dependency dd but for the version.
    /// Used by the how-to-install command, which calls compute_how_to_install().
    pub fn dep_satisfied_by_wrong_version(&self, dd:&Dependency) -> Vec<&str> {
        assert! (self.dep_is_satisfied(dd).is_none());
        let mut result = vec![];
        // another loop on dd
        for dep in dd {
            match self.installed_debvers.get(&dep.package_num) {
                None => (),
                Some(v) => {
                    (result.push(self.get_package_name(dep.package_num)));
                }
            };
        }
        return result;
    }
}


