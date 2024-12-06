use crate::config::Cli;

mod append;
mod build;
mod erase;
mod merge;

pub(crate) fn procedure(config: &Cli) -> Result<(), std::io::Error> {
    match config {
        Cli::Append(args) => append::append(args),
        Cli::Build(args) => build::build(args),
        Cli::Erase(args) => erase::erase(args),
        Cli::Merge(args) => merge::merge(args),
    }
}

fn unique_fold<T>(mut acc: Vec<T>, current: &T) -> Vec<T>
where
    T: PartialEq + Clone,
{
    if !acc.iter().any(|v| v == current) {
        acc.push(current.clone());
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unique_fold {
        use super::*;

        #[test]
        fn pushing_when_unique() {
            let acc = vec!["a", "b"];
            let current = "c";
            let result = unique_fold(acc, &current);
            assert_eq!(result, vec!["a", "b", "c"]);
        }

        #[test]
        fn not_pushing_when_not_unique() {
            let acc = vec!["a", "b"];
            let current = "b";
            let result = unique_fold(acc, &current);
            assert_eq!(result, vec!["a", "b"]);
        }
    }
}
