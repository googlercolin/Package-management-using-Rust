use crate::Packages;
use crate::packages::Dependency;
use itertools::Itertools;

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

    /// Computes a set of packages that need to be installed to satisfy package_name's deps given the current installed packages.
    /// When a dependency A | B | C is unsatisfied, there are two possible cases:
    ///   (1) there are no versions of A, B, or C installed; pick the alternative with the highest version number (yes, compare apples and oranges).
    ///   (2) at least one of A, B, or C is installed (say A, B), but with the wrong version; of the installed packages (A, B), pick the one with the highest version number.
    pub fn compute_how_to_install(&self, package_name: &str) -> Vec<i32> {
        if !self.package_exists(package_name) {
            return vec![];
        }
        let mut dependencies_to_add : Vec<i32> = vec![];

        // implement more sophisticated worklist

        return dependencies_to_add;
    }
}
