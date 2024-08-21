// examples of versions: "2.3", "3.1", "20.2.0.7"
fn split_version(version: &str) -> Vec<u32> {
    version.split('.').map(|s| s.parse().unwrap()).collect()
}

fn version_gte(version: &[u32], other: &[u32]) -> bool {
    for (v, o) in version.iter().zip(other.iter()) {
        if v == o {
            continue;
        }
        return v > o;
    }
    true
}
fn version_lte(version: &[u32], other: &[u32]) -> bool {
    for (v, o) in version.iter().zip(other.iter()) {
        if v == o {
            continue;
        }
        return v < o;
    }
    true
}

pub(crate) fn is_in_version_range(
    version: &str,
    range: (&Option<String>, &Option<String>),
) -> bool {
    let version_split = split_version(version);

    // If the version is less than "since", it's out of range
    if let Some(since) = range.0 {
        let since_split = split_version(since);
        if !version_gte(&version_split, &since_split) {
            return false;
        }
    }

    // If the version is more than "until", it's out of range
    if let Some(until) = range.1 {
        let until_split = split_version(until);
        if !version_lte(&version_split, &until_split) {
            return false;
        }
    }

    true
}
