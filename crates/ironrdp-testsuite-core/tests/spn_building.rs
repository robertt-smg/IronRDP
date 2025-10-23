use ironrdp_connector::ServerName;

#[test]
fn spn_for_hostname() {
    let sn = ServerName::new("rdp.example.com").into_inner();
    let spn = format!("TERMSRV/{}", sn);
    assert_eq!(spn, "TERMSRV/rdp.example.com");
}

#[test]
fn spn_for_ipv4() {
    let sn = ServerName::new("10.0.0.5").into_inner();
    let spn = format!("TERMSRV/{}", sn);
    assert_eq!(spn, "TERMSRV/10.0.0.5");
}

#[test]
fn spn_for_ipv6() {
    let sn = ServerName::new("2001:db8::8a2e:370:7334").into_inner();
    let spn = format!("TERMSRV/{}", sn);
    assert_eq!(spn, "TERMSRV/2001:db8::8a2e:370:7334");
}

#[test]
fn spn_strips_port() {
    let sn = ServerName::new("rdp.example.com:3389").into_inner();
    let spn = format!("TERMSRV/{}", sn);
    assert_eq!(spn, "TERMSRV/rdp.example.com");
}
