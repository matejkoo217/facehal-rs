// Minimal IFace client probe: mimics what system_server's
// FaceStartUserClient.startHalOperation() does, so we can see the exact
// binder::Status returned by the running HAL without touching the framework.
use android_hardware_biometrics_face::aidl::android::hardware::biometrics::face::{
    IFace::IFace,
    ISessionCallback::{BnSessionCallback, ISessionCallback},
    AuthenticationFrame::AuthenticationFrame,
    EnrollmentFrame::EnrollmentFrame,
    Error::Error,
    Feature::Feature,
};
use android_hardware_keymaster::aidl::android::hardware::keymaster::HardwareAuthToken::HardwareAuthToken;
use binder::{BinderFeatures, Interface, Status, Strong};

struct Cb;
impl Interface for Cb {}
impl ISessionCallback for Cb {
    fn r#onChallengeGenerated(&self, _c: i64) -> binder::Result<()> { Ok(()) }
    fn r#onChallengeRevoked(&self, _c: i64) -> binder::Result<()> { Ok(()) }
    fn r#onAuthenticationFrame(&self, _f: &AuthenticationFrame) -> binder::Result<()> { Ok(()) }
    fn r#onEnrollmentFrame(&self, _f: &EnrollmentFrame) -> binder::Result<()> { Ok(()) }
    fn r#onError(&self, e: Error, vc: i32) -> binder::Result<()> { eprintln!("  cb.onError({e:?}, {vc})"); Ok(()) }
    fn r#onEnrollmentProgress(&self, _id: i32, _r: i32) -> binder::Result<()> { Ok(()) }
    fn r#onAuthenticationSucceeded(&self, _id: i32, _h: &HardwareAuthToken) -> binder::Result<()> { Ok(()) }
    fn r#onAuthenticationFailed(&self) -> binder::Result<()> { Ok(()) }
    fn r#onLockoutTimed(&self, _d: i64) -> binder::Result<()> { Ok(()) }
    fn r#onLockoutPermanent(&self) -> binder::Result<()> { Ok(()) }
    fn r#onLockoutCleared(&self) -> binder::Result<()> { Ok(()) }
    fn r#onInteractionDetected(&self) -> binder::Result<()> { Ok(()) }
    fn r#onEnrollmentsEnumerated(&self, _ids: &[i32]) -> binder::Result<()> { Ok(()) }
    fn r#onFeaturesRetrieved(&self, _f: &[Feature]) -> binder::Result<()> { Ok(()) }
    fn r#onFeatureSet(&self, _f: Feature) -> binder::Result<()> { Ok(()) }
    fn r#onEnrollmentsRemoved(&self, _ids: &[i32]) -> binder::Result<()> { Ok(()) }
    fn r#onAuthenticatorIdRetrieved(&self, _id: i64) -> binder::Result<()> { Ok(()) }
    fn r#onAuthenticatorIdInvalidated(&self, _id: i64) -> binder::Result<()> { Ok(()) }
    fn r#onSessionClosed(&self) -> binder::Result<()> { Ok(()) }
}

fn main() {
    let name = "android.hardware.biometrics.face.IFace/default";
    let svc = binder::get_interface::<dyn IFace>(name).expect("get_interface failed");
    eprintln!("getInterfaceVersion = {:?}", svc.r#getInterfaceVersion());
    match svc.r#getInterfaceHash() {
        Ok(h) => eprintln!("getInterfaceHash    = {h}"),
        Err(e) => eprintln!("getInterfaceHash ERR = {e:?}"),
    }
    match svc.r#getSensorProps() {
        Ok(props) => {
            eprintln!("getSensorProps OK: {} props", props.len());
            for p in &props {
                eprintln!(
                    "  sensorId={} strength={:?} type={:?} maxEnroll={}",
                    p.commonProps.sensorId, p.commonProps.sensorStrength, p.sensorType,
                    p.commonProps.maxEnrollmentsPerUser
                );
            }
        }
        Err(e) => eprintln!("getSensorProps ERR = {e:?}"),
    }
    let cb: Strong<dyn ISessionCallback> =
        BnSessionCallback::new_binder(Cb, BinderFeatures::default());
    match svc.r#createSession(1, 0, &cb) {
        Ok(_) => eprintln!("createSession(1,0) => OK"),
        Err(e) => eprintln!(
            "createSession(1,0) => ERR status={e:?} exception={:?} service_specific={:?}",
            e.exception_code(),
            e.service_specific_error()
        ),
    }
    let _ = Status::ok();
}
