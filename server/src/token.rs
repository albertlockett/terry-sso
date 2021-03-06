use jwt_simple::prelude::*;

const PEM: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDx/8TgQlGuvyOC
fWpEktusmOkUq5zSFpFv0cWlzl8l4GrbF8mm7mCe4qwUie3m80kvVmGjXtUKDQtg
LdfMIdTLCVB+MyTMDUGtKsMPnzLHh5bBKUjJHqv12KQlgmboejlzj7AIqBr7ZgWu
AFmn0EpQccJZuLyi0T6t08v/wr6bGeq39Hg+bZC8UFGsnjfebmdnHTbgWv0Mw/n1
KXRCM4J/HNavfcm7KvmdRFTGXQhxKD8GM+pA3iPypN4AMsH8ERBW98m4FzV3LQik
4iGeuQz0aTQZ8J48pi8UIef3TJE5m+uK6RwUwVo1C1s2QQevZSHhVUNgj/wWAcZJ
ao+vj0kbAgMBAAECggEAaRgHOtDyQi5V0HyAoukYJ1t8lXJ1tzL/1AzIhdoargtI
vH8XHryVD0snKgBAZbSvyrP70QQFBsllcE5YUhk7HTWdVEEHsOns5LQWY/liBw7W
JGq2ZklAqpJ7rqJz6G/z9UCoVQoUNas3ujoCtYMaUtckbOxyV08BpmG9TQQY3KfL
ahsPY48fNcRsNG3hwgdpcja9G0OCXl4uZbKNfBmYfnEEFpGRiYyk4NmpOEDQVLGJ
AcA4sEjxMAx9dbF7RSvFXIvxP2OuUsrN3iljitOYdILzYftlKquD7KKDlcvF7frD
XILlwXH1VR2AvnLufYZtHZ5ek9TBTo1m2mwFXNd76QKBgQD/bra3Kg49KCLB2cK0
heBPFfAGnTbdv+0L8HtTAaWUJs/SN12c1qO70rBNv4sgDnludlN6KsDgcfFWRhXh
Rq8lIU/3+DGKp4kkQfBZUYmgiP69Mdc2Ii+S0XG3OLTpz6LkFDoIVL62UOWavR5G
zvLbC/IFTp1k4RulY9TBnnybrQKBgQDyiWojkRI5tfvNkvU36qfOhwR7/Tpuvjaw
rNHdTMJ0grBHZyxzx4S++D1jpYFauO0GcnPpe/KGelOvt1DixX3jQV98O78ne7fI
WaHwRbWYjASr8wnzyp6hulwRw4SvvICj4U41ysninroz5PoWsRJcGMWnBOuS0H41
VaJT3XQQ5wKBgQDzeJBo3olb+dnHdM2GTOWV/I8hftFBZSOJ8dtbL4KoSKZm9ach
S+YdraGnOiBeme1dQSQomn/mcDVdysgoRrvDZJFZBxYR18d86+0w9et9PZeytsMw
Mj2ucD04K7El/GiDhlQahMyT82AnsoQdcj1CVYyHSnv0IC294/5z36FesQKBgQDe
5KSTnS43tWR9L9KGmfOv3OulGzrJLaxLP/f6viFzO+vPIyV6HeGEM9UPL7oqawsB
wst9xzhCmEcylHEUh1VDNgAPx0TMf2mOCbnGe/oBAKUGdWwvDYYtsXwNozbc3osG
kPf0fbJ9v0v2hzfDBOaPlwCGlwH8QfnLADEXi8yv/wKBgCM+gR3CU2IsNHFbgBEf
oJTrErjZSZGr8EF4Dv2xrpTmEOKauqqZTFqt6QoioRwmf3sW5b7gW5+RPho8HYL3
vnaKwEHkKoqrgSDMOfvZ3CHPYVQq8yPbiEjRd6p9NQ+LbJPw0n3CPnJGM5tb2ECi
aHxF0FjYkTeifOuHa3kFzxZ4
-----END PRIVATE KEY-----";

#[derive(Serialize, Deserialize)]
struct CustomClaims {
    scopes: String,
}

pub fn generate_token(subject: &str, audience: &str, scopes: &str) -> String {
    let key_pair = RS256KeyPair::from_pem(PEM).unwrap();

    let exp = Duration::from_millis(30 * 60 * 1000);
    let custom_claims = CustomClaims {
        scopes: String::from(scopes),
    };
    let token = key_pair
        .sign(
            Claims::with_custom_claims(custom_claims, exp)
                .with_subject(subject)
                .with_audience(audience)
                .with_issuer("terrylockett.ca"),
        )
        .unwrap();
    return token;
}
