#[allow(unused_macros)]
macro_rules! iwo {
    ($o:ident, $i:expr; $e:tt) => {
		#[allow(unused_braces)]
        $e
        #[allow(unused_assignments)]

        {
            $o += $i;
        }
    };
}
#[allow(unused_imports)]
pub(crate) use iwo;

#[allow(unused_macros)]
macro_rules! to_exact {
    ($e:expr;$l:literal) => {{
        let mut data: [u8; $l] = [0; $l];
        for i in 0..$l {
            data[i] = $e[i];
        }
        data
    }};
}
#[allow(unused_imports)]
pub(crate) use to_exact;
