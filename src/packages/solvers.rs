use crate::Packages;
use crate::packages::Dependency;
use std::collections::VecDeque;
use rpkg::debversion;

impl Packages {
    /// Computes a solution for the transitive dependencies of package_name; when there is a choice A | B | C, 
    /// chooses the first option A. Returns a Vec<i32> of package numbers.
    ///
    /// Note: does not consider which packages are installed.
    pub fn transitive_dep_solution(&self, package_name: &str) -> Vec<i32> {
        if !self.package_exists(package_name) {
            return vec![];
        }

        let deps : &Vec<Dependency> = &*self.dependencies.get(self.get_package_num(package_name)).unwrap();
        let mut dependency_set:Vec<i32> = vec![];

        // implement worklist

        for dep in deps {
            dependency_set.push(dep.get(0).unwrap().package_num)
        }

        let mut i = 0;
        while let Some(pkg_num)= dependency_set.get(i) {
            if let Some(deps) = self.dependencies.get(pkg_num){
                for dep in deps {
                    let dep_num = dep.get(0).unwrap().package_num;
                    if !dependency_set.contains(&dep_num) {
                        dependency_set.push(dep_num);
                        // println!("{:?}", dependency_set);
                    }
                }
            };
            i = i + 1;
        }
        return dependency_set;
    }

    /// Computes a set of packages that need to be installed to satisfy package_name's deps given
    /// the current installed packages.
    /// When a dependency A | B | C is unsatisfied, there are two possible cases:
    ///   (1) there are no versions of A, B, or C installed; pick the alternative with the
    /// highest version number (yes, compare apples and oranges).
    ///   (2) at least one of A, B, or C is installed (say A, B), but with the wrong version;
    /// of the installed packages (A, B), pick the one with the highest version number.
    pub fn compute_how_to_install(&self, package_name: &str) -> Vec<i32> {
        if !self.package_exists(package_name) {
            return vec![];
        }
        let mut dependencies_to_add : Vec<i32> = vec![];

        // implement more sophisticated worklist

        let deps : &Vec<Dependency> = &*self.dependencies.get(self.get_package_num(package_name)).unwrap();
        let mut dependency_queue = VecDeque::new();
        for dep in deps {
            dependency_queue.push_back(dep);
        }

        while let Some(pkgs) = dependency_queue.pop_front() {
            // dep satisfied
            if self.dep_is_satisfied(pkgs).is_some() {
                // let mut dep_to_queue: Dependency = vec![];
                // for i in 0..pkgs.len() {
                //     let pkg = pkgs.get(i).unwrap();
                //     if pkg.package_num == *self.get_package_num(self.dep_is_satisfied(pkgs).unwrap()) {
                //         dep_to_queue.push(*pkg);
                //     }
                // }
                // dependency_queue.push_back(&dep_to_queue);
                continue;
            }
            // dep satisfied, wrong ver
            let wrong_ver_deps = self.dep_satisfied_by_wrong_version(pkgs);
            if wrong_ver_deps.len()!=0 {
                let first = wrong_ver_deps.first().unwrap();
                let mut dep_to_add = self.get_package_num(first);
                let mut max = self.get_available_debver(first).unwrap();
                for wrong_ver_dep in wrong_ver_deps {
                    if let ver_of_wrong_ver_dep = self.get_available_debver(wrong_ver_dep).unwrap()  {
                        if debversion::cmp_debversion_with_op(&debversion::VersionRelation::StrictlyGreater, ver_of_wrong_ver_dep, max) {
                            max = ver_of_wrong_ver_dep;
                            dep_to_add = self.get_package_num(wrong_ver_dep);
                        }
                    }
                }

                for dep in self.dependencies.get(dep_to_add).unwrap() {
                    dependency_queue.push_back(dep);
                }

                if !dependencies_to_add.contains(dep_to_add) {
                    dependencies_to_add.push(*dep_to_add);
                }
                continue;
            }
            // none installed
            else {
                // let mut dep_to_add = pkgs.front().unwrap();
                // let mut highest_ver = self.get_available_debver(self.get_package_name(dep_to_add.package_num)).unwrap();
                // for i in 0..pkgs.len() {
                //     let option = pkgs.get(i).unwrap();
                //     match self.get_available_debver(self.get_package_name(option.package_num)) {
                //         Some(x) => {
                //             if debversion::cmp_debversion_with_op(&debversion::VersionRelation::StrictlyGreater, x, highest_ver) {
                //                 dep_to_add = option;
                //                 highest_ver = x;
                //             }
                //         }
                //         None => ()
                //     }
                // }
                // for i in 0..pkgs.len() {
                //     let pkg = pkgs.get(i).unwrap();
                //     if pkg.package_num == dep_to_add.package_num {
                //         dep_to_queue.push(*pkg);
                //     }
                // }
                // dependency_queue.push_back(&dep_to_queue);
                // dependencies_to_add.push(dep_to_add.package_num);

                // continue;
                let mut not_inst_deps = Vec::new();
                for dep in pkgs {
                    not_inst_deps.push(dep.package_num);
                }
                let first = not_inst_deps.first().unwrap();
                let mut dep_to_add = first;
                let mut max = self.get_available_debver(self.get_package_name(*first)).unwrap();
                for not_inst_dep in not_inst_deps.iter() {
                    if let ver_of_not_inst_dep = self.get_available_debver(self.get_package_name(*not_inst_dep)).unwrap()  {
                        if debversion::cmp_debversion_with_op(&debversion::VersionRelation::StrictlyGreater, ver_of_not_inst_dep, max) {
                            max = ver_of_not_inst_dep;
                            dep_to_add = &not_inst_dep;
                        }
                    }
                }

                for dep in self.dependencies.get(dep_to_add).unwrap() {
                    dependency_queue.push_back(dep);
                }

                if !dependencies_to_add.contains(dep_to_add) {
                    dependencies_to_add.push(*dep_to_add);
                }
                continue;
            }
        }

        return dependencies_to_add;
    }
}
