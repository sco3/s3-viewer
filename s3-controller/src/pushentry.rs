use aws_sdk_s3::primitives::DateTime;

use crate::keyinfo::KeyInfo;

pub(crate) fn push_entry(all_keys: &mut Vec<KeyInfo>, obj: aws_sdk_s3::types::Object) {
    let zero_dt = DateTime::from_millis(0);
    let name = obj.key.unwrap_or("".into());
    let last_modi = obj.last_modified.unwrap_or(zero_dt);
    let size = obj.size.unwrap_or(0);

    let info = KeyInfo {
        key: name,
        last_modified: last_modi.to_string(),
        size: size,
        last_modified_dt: last_modi,
    };
    all_keys.push(info);
}
