use std::cell::Cell;
use witx_bindgen_wasmtime::imports::{InBuffer, OutBuffer};
use witx_bindgen_wasmtime::{BorrowChecker, GuestPtr, Table};

witx_bindgen_wasmtime::import!("tests/host.witx");

#[derive(Default)]
pub struct Host {
    scalar: Cell<u32>,
    borrow_checker: BorrowChecker,
    host_state_table: Table<SuchState>,
    host_state2_table: Table<()>,
    host_state2_closed: Cell<bool>,
}

pub struct SuchState(u32);

// TODO: implement propagation of errors instead of `unwrap()` everywhere

impl Host for host::Host {
    type HostState = SuchState;
    type HostState2 = ();

    fn host_state_table(&self) -> &Table<SuchState> {
        &self.host_state_table
    }

    fn host_state2_table(&self) -> &Table<()> {
        &self.host_state2_table
    }

    fn borrow_checker(&self) -> &BorrowChecker {
        &self.borrow_checker
    }

    fn roundtrip_u8(&self, val: u8) -> u8 {
        val
    }

    fn roundtrip_s8(&self, val: i8) -> i8 {
        val
    }

    fn roundtrip_u16(&self, val: u16) -> u16 {
        val
    }

    fn roundtrip_s16(&self, val: i16) -> i16 {
        val
    }

    fn roundtrip_u32(&self, val: u32) -> u32 {
        val
    }

    fn roundtrip_s32(&self, val: i32) -> i32 {
        val
    }

    fn roundtrip_u64(&self, val: u64) -> u64 {
        val
    }

    fn roundtrip_s64(&self, val: i64) -> i64 {
        val
    }

    fn roundtrip_usize(&self, val: u32) -> u32 {
        val
    }

    fn roundtrip_f32(&self, val: f32) -> f32 {
        val
    }

    fn roundtrip_f64(&self, val: f64) -> f64 {
        val
    }

    fn roundtrip_char(&self, val: char) -> char {
        val
    }

    fn multiple_results(&self) -> (u8, u16) {
        (4, 5)
    }

    fn set_scalar(&self, val: u32) {
        self.scalar.set(val);
    }

    fn get_scalar(&self) -> u32 {
        self.scalar.get()
    }

    fn swap_tuple(&self, a: (u8, u32)) -> (u32, u8) {
        (a.1, a.0)
    }

    fn roundtrip_flags1(&self, a: F1) -> F1 {
        drop(a.to_string());
        drop(format!("{:?}", a));
        drop(a & F1::all());
        a
    }

    fn roundtrip_flags2(&self, a: F2) -> F2 {
        a
    }

    fn roundtrip_record1(&self, a: R1) -> R1 {
        drop(format!("{:?}", a));
        a
    }

    fn tuple0(&self, _: ()) {}

    fn tuple1(&self, a: (u8,)) -> (u8,) {
        (a.0,)
    }

    fn roundtrip_option(&self, a: Option<f32>) -> Option<u8> {
        a.map(|x| x as u8)
    }

    fn roundtrip_result(&self, a: Result<u32, f32>) -> Result<f64, u8> {
        match a {
            Ok(a) => Ok(a.into()),
            Err(b) => Err(b as u8),
        }
    }

    fn roundtrip_enum(&self, a: E1) -> E1 {
        assert_eq!(a, a);
        a
    }

    fn invert_bool(&self, a: bool) -> bool {
        !a
    }

    fn variant_casts(&self, a: Casts) -> Casts {
        a
    }

    fn variant_zeros(&self, a: Zeros) -> Zeros {
        a
    }

    fn variant_typedefs(&self, _: Option<u32>, _: bool, _: Result<u32, ()>) {}

    fn legacy_params(
        &self,
        a: (u32, u32),
        _: R1,
        _: (u8, i8, u16, i16, u32, i32, u64, i64, f32, f64),
    ) {
        assert_eq!(a, (1, 2));
    }

    fn legacy_result(&self, succeed: bool) -> Result<LegacyResult, E1> {
        if succeed {
            Ok((
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9.,
                10.,
                R1 {
                    a: 0,
                    b: F1::empty(),
                },
            ))
        } else {
            Err(E1::B)
        }
    }

    fn list_param(&self, ptr: GuestPtr<'_, [u8]>) {
        let list = ptr.borrow().unwrap();
        assert_eq!(*list, [1, 2, 3, 4]);
        assert!(ptr.borrow().is_ok());
        assert!(ptr.borrow_mut().is_err());
        drop(list);
        assert!(ptr.borrow().is_ok());
        assert!(ptr.borrow_mut().is_ok());
    }

    fn list_param2(&self, ptr: GuestPtr<'_, str>) {
        assert_eq!(&*ptr.borrow().unwrap(), "foo");
    }

    fn list_param3(&self, ptr: Vec<GuestPtr<'_, str>>) {
        assert_eq!(ptr.len(), 3);
        assert_eq!(&*ptr[0].borrow().unwrap(), "foo");
        assert_eq!(&*ptr[1].borrow().unwrap(), "bar");
        assert_eq!(&*ptr[2].borrow().unwrap(), "baz");
    }

    fn list_param4(&self, ptr: Vec<Vec<GuestPtr<'_, str>>>) {
        assert_eq!(ptr.len(), 2);
        assert_eq!(&*ptr[0][0].borrow().unwrap(), "foo");
        assert_eq!(&*ptr[0][1].borrow().unwrap(), "bar");
        assert_eq!(&*ptr[1][0].borrow().unwrap(), "baz");
    }

    fn list_result(&self) -> Vec<u8> {
        vec![1, 2, 3, 4, 5]
    }

    fn list_result2(&self) -> String {
        "hello!".to_string()
    }

    fn list_result3(&self) -> Vec<String> {
        vec!["hello,".to_string(), "world!".to_string()]
    }

    fn list_in_record1(&self, ty: ListInRecord1<'_>) {
        assert_eq!(&*ty.a.borrow().unwrap(), "list_in_record1");
    }

    fn list_in_record2(&self) -> ListInRecord2 {
        ListInRecord2 {
            a: "list_in_record2".to_string(),
        }
    }

    fn list_in_record3(&self, a: ListInRecord3Param<'_>) -> ListInRecord3Result {
        assert_eq!(&*a.a.borrow().unwrap(), "list_in_record3 input");
        ListInRecord3Result {
            a: "list_in_record3 output".to_string(),
        }
    }

    fn list_in_record4(&self, a: ListInAliasParam<'_>) -> ListInAliasResult {
        assert_eq!(&*a.a.borrow().unwrap(), "input4");
        ListInRecord4Result {
            a: "result4".to_string(),
        }
    }

    fn list_in_variant1(
        &self,
        a: ListInVariant11<'_>,
        b: ListInVariant12<'_>,
        c: ListInVariant13<'_>,
    ) {
        assert_eq!(&*a.unwrap().borrow().unwrap(), "foo");
        assert_eq!(&*b.unwrap_err().borrow().unwrap(), "bar");
        match c {
            ListInVariant13::V0(s) => assert_eq!(&*s.borrow().unwrap(), "baz"),
            ListInVariant13::V1(_) => panic!(),
        }
    }

    fn list_in_variant2(&self) -> Option<String> {
        Some("list_in_variant2".to_string())
    }

    fn list_in_variant3(&self, a: ListInVariant3Param<'_>) -> Option<String> {
        assert_eq!(&*a.unwrap().borrow().unwrap(), "input3");
        Some("output3".to_string())
    }

    fn errno_result(&self) -> Result<(), MyErrno> {
        MyErrno::A.to_string();
        format!("{:?}", MyErrno::A);
        fn assert_error<T: std::error::Error>() {}
        assert_error::<MyErrno>();
        Err(MyErrno::B)
    }

    fn list_typedefs(
        &self,
        a: ListTypedef<'_>,
        b: ListTypedef3Param<'_>,
    ) -> (ListTypedef2, ListTypedef3Result) {
        assert_eq!(&*a.borrow().unwrap(), "typedef1");
        assert_eq!(b.len(), 1);
        assert_eq!(&*b[0].borrow().unwrap(), "typedef2");
        (b"typedef3".to_vec(), vec!["typedef4".to_string()])
    }

    fn host_state_create(&self) -> SuchState {
        SuchState(100)
    }

    fn host_state_get(&self, state: &SuchState) -> u32 {
        state.0
    }

    fn host_state2_create(&self) {}

    fn host_state2_saw_close(&self) -> bool {
        self.host_state2_closed.get()
    }

    fn host_state2_close(&self, _state: ()) {
        self.host_state2_closed.set(true);
    }

    fn two_host_states(&self, _a: &SuchState, _b: &()) -> (SuchState, ()) {
        (SuchState(2), ())
    }

    fn host_state2_param_record(&self, _a: HostStateParamRecord<'_, Self>) {}
    fn host_state2_param_tuple(&self, _a: (&'_ (),)) {}
    fn host_state2_param_option(&self, _a: Option<&'_ ()>) {}
    fn host_state2_param_result(&self, _a: Result<&'_ (), u32>) {}
    fn host_state2_param_variant(&self, _a: HostStateParamVariant<'_, Self>) {}
    fn host_state2_param_list(&self, _a: Vec<&()>) {}

    fn host_state2_result_record(&self) -> HostStateResultRecord<Self> {
        HostStateResultRecord { a: () }
    }
    fn host_state2_result_tuple(&self) -> ((),) {
        ((),)
    }
    fn host_state2_result_option(&self) -> Option<()> {
        Some(())
    }
    fn host_state2_result_result(&self) -> Result<(), u32> {
        Ok(())
    }
    fn host_state2_result_variant(&self) -> HostStateResultVariant<Self> {
        HostStateResultVariant::V0(())
    }
    fn host_state2_result_list(&self) -> Vec<()> {
        vec![(), ()]
    }

    fn buffer_u8(&self, in_: GuestPtr<'_, [u8]>, out: GuestPtr<'_, [u8]>) -> u32 {
        let in_ = in_.borrow().unwrap();
        assert_eq!(*in_, [0]);
        let mut out = out.borrow_mut().unwrap();
        assert_eq!(out.len(), 10);
        out[0] = 1;
        out[1] = 2;
        out[2] = 3;
        3
    }

    fn buffer_u32(&self, in_: GuestPtr<'_, [u32]>, out: GuestPtr<'_, [u32]>) -> u32 {
        let in_ = in_.borrow().unwrap();
        assert_eq!(*in_, [0]);
        let mut out = out.borrow_mut().unwrap();
        assert_eq!(out.len(), 10);
        out[0] = 1;
        out[1] = 2;
        out[2] = 3;
        3
    }

    fn buffer_bool(&self, in_: InBuffer<'_, bool>, mut out: OutBuffer<'_, bool>) -> u32 {
        assert!(in_.len() < out.capacity());
        let len = in_.len();
        for item in in_.iter().unwrap() {
            let item = item.unwrap();
            out.write(Some(!item)).unwrap();
        }
        len as u32
    }

    fn buffer_string(
        &self,
        in_: InBuffer<'_, GuestPtr<'_, str>>,
        mut out: OutBuffer<'_, String>,
    ) -> u32 {
        assert!(in_.len() < out.capacity());
        let len = in_.len();
        for item in in_.iter().unwrap() {
            let item = item.unwrap();
            let s = item.borrow().unwrap();
            out.write(Some(s.to_uppercase())).unwrap();
        }
        len as u32
    }

    fn buffer_list_bool(
        &self,
        in_: InBuffer<'_, Vec<bool>>,
        mut out: OutBuffer<'_, Vec<bool>>,
    ) -> u32 {
        assert!(in_.len() < out.capacity());
        let len = in_.len();
        for item in in_.iter().unwrap() {
            let item = item.unwrap();
            out.write(Some(item.into_iter().map(|b| !b).collect()))
                .unwrap();
        }
        len as u32
    }

    fn buffer_buffer_bool(&self, in_: InBuffer<'_, InBuffer<'_, bool>>) {
        assert_eq!(in_.len(), 1);
        let buf = in_.iter().unwrap().next().unwrap().unwrap();
        assert_eq!(buf.len(), 5);
        assert_eq!(
            buf.iter()
                .unwrap()
                .collect::<Result<Vec<bool>, _>>()
                .unwrap(),
            [true, false, true, true, false]
        );
    }

    fn buffer_mutable1(&self, a: Vec<InBuffer<'_, bool>>) {
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].len(), 5);
        assert_eq!(
            a[0].iter().unwrap().collect::<Result<Vec<_>, _>>().unwrap(),
            [true, false, true, true, false]
        );
    }

    fn buffer_mutable2(&self, a: Vec<GuestPtr<'_, [u8]>>) -> u32 {
        assert_eq!(a.len(), 1);
        assert!(a[0].len() > 4);
        let mut view = a[0].borrow_mut().unwrap();
        view[..4].copy_from_slice(&[1, 2, 3, 4]);
        return 4;
    }

    fn buffer_mutable3(&self, mut a: Vec<OutBuffer<'_, bool>>) -> u32 {
        assert_eq!(a.len(), 1);
        assert!(a[0].capacity() > 3);
        a[0].write([false, true, false].iter().copied()).unwrap();
        return 3;
    }

    fn buffer_in_record(&self, _: BufferInRecord<'_>) {}
    fn buffer_typedef(
        &self,
        _: ParamInBufferU8<'_>,
        _: ParamOutBufferU8<'_>,
        _: ParamInBufferBool<'_>,
        _: ParamOutBufferBool<'_>,
    ) {
    }
}
