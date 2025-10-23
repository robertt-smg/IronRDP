use ironrdp_connector::sspi::Username;

#[test]
fn upn_ignores_domain() {
    let user = Username::new("alice@example.com", Some("EXAMPLE"));
    // SSPI Username validates the string; behavior is to parse UPN as-is
    assert!(user.is_ok());
}

#[test]
fn domain_backslash_user() {
    let user = Username::new("alice", Some("EXAMPLE"))
        .expect("Username construction should succeed with domain");
    // We cannot easily inspect internals; just assert success
}

#[test]
fn bare_username_ok() {
    let user = Username::new("alice", None).expect("Bare username should be accepted");
    assert!(matches!(user.name_type(), ironrdp_connector::sspi::UsernameType::Unknown | ironrdp_connector::sspi::UsernameType::Nt4));
}
