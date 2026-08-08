use rsp_core::{
    ApplicationId, ChannelName, ChannelRevision, ContentDigest, ContentLength, RSP_V1, ReleaseId,
    ReleaseVersion, TargetId,
};
use rsp_discovery::ChannelState;
use rsp_json::to_string;
use rsp_manifest::{ManifestFile, ManifestPath, ReleaseManifest};
use rsp_release::ReleaseDescriptor;
use std::collections::BTreeMap;

fn digest(fill: char) -> ContentDigest {
    ContentDigest::sha256(std::iter::repeat_n(fill, 64).collect::<String>()).unwrap()
}

#[test]
fn channel_state_has_stable_canonical_bytes() {
    let state = ChannelState {
        protocol: RSP_V1,
        application: ApplicationId("desktop".into()),
        channel: ChannelName("stable".into()),
        revision: ChannelRevision(u64::MAX),
        release: digest('a'),
    };

    assert_eq!(
        to_string(&state).unwrap(),
        concat!(
            "{\"application\":\"desktop\",",
            "\"channel\":\"stable\",",
            "\"protocol\":{\"major\":1,\"minor\":0},",
            "\"release\":\"sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",",
            "\"revision\":\"18446744073709551615\"}"
        )
    );
}

#[test]
fn release_descriptor_canonicalizes_target_map() {
    let mut targets = BTreeMap::new();
    targets.insert(TargetId("win-x64".into()), digest('b'));
    targets.insert(TargetId("linux-x64".into()), digest('c'));

    let release = ReleaseDescriptor {
        protocol: RSP_V1,
        application: ApplicationId("desktop".into()),
        release: ReleaseId("release-42".into()),
        version: ReleaseVersion("8.12.0".into()),
        targets,
    };

    assert_eq!(
        to_string(&release).unwrap(),
        concat!(
            "{\"application\":\"desktop\",",
            "\"protocol\":{\"major\":1,\"minor\":0},",
            "\"release\":\"release-42\",",
            "\"targets\":{",
            "\"linux-x64\":\"sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\",",
            "\"win-x64\":\"sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"},",
            "\"version\":\"8.12.0\"}"
        )
    );
}

#[test]
fn manifest_uses_canonical_paths_and_string_content_lengths() {
    let mut files = BTreeMap::new();
    files.insert(
        ManifestPath::new("bin/app.exe").unwrap(),
        ManifestFile {
            content: digest('d'),
            size: ContentLength(u64::MAX),
            executable: false,
        },
    );

    let manifest = ReleaseManifest {
        protocol: RSP_V1,
        target: TargetId("win-x64".into()),
        files,
    };

    assert_eq!(
        to_string(&manifest).unwrap(),
        concat!(
            "{\"files\":{\"bin/app.exe\":{",
            "\"content\":\"sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\",",
            "\"executable\":false,",
            "\"size\":\"18446744073709551615\"}},",
            "\"protocol\":{\"major\":1,\"minor\":0},",
            "\"target\":\"win-x64\"}"
        )
    );
}
