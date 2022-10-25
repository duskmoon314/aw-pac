#[doc = "Register `dbi_ctl_0` reader"]
pub struct R(crate::R<DBI_CTL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_CTL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_CTL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_CTL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dbi_ctl_0` writer"]
pub struct W(crate::W<DBI_CTL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CTL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DBI_CTL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CTL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vi_src_type` reader - Video Source Type"]
pub type VI_SRC_TYPE_R = crate::BitReader<VI_SRC_TYPE_A>;
#[doc = "Video Source Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VI_SRC_TYPE_A {
    #[doc = "0: `0`"]
    RGB32 = 0,
    #[doc = "1: `1`"]
    RGB16 = 1,
}
impl From<VI_SRC_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: VI_SRC_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl VI_SRC_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VI_SRC_TYPE_A {
        match self.bits {
            false => VI_SRC_TYPE_A::RGB32,
            true => VI_SRC_TYPE_A::RGB16,
        }
    }
    #[doc = "Checks if the value of the field is `RGB32`"]
    #[inline(always)]
    pub fn is_rgb32(&self) -> bool {
        *self == VI_SRC_TYPE_A::RGB32
    }
    #[doc = "Checks if the value of the field is `RGB16`"]
    #[inline(always)]
    pub fn is_rgb16(&self) -> bool {
        *self == VI_SRC_TYPE_A::RGB16
    }
}
#[doc = "Field `vi_src_type` writer - Video Source Type"]
pub type VI_SRC_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, VI_SRC_TYPE_A, O>;
impl<'a, const O: u8> VI_SRC_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb32(self) -> &'a mut W {
        self.variant(VI_SRC_TYPE_A::RGB32)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgb16(self) -> &'a mut W {
        self.variant(VI_SRC_TYPE_A::RGB16)
    }
}
#[doc = "Field `element_a_pos` reader - Element A Position"]
pub type ELEMENT_A_POS_R = crate::BitReader<ELEMENT_A_POS_A>;
#[doc = "Element A Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELEMENT_A_POS_A {
    #[doc = "0: `0`"]
    _31_24 = 0,
    #[doc = "1: `1`"]
    _7_0 = 1,
}
impl From<ELEMENT_A_POS_A> for bool {
    #[inline(always)]
    fn from(variant: ELEMENT_A_POS_A) -> Self {
        variant as u8 != 0
    }
}
impl ELEMENT_A_POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELEMENT_A_POS_A {
        match self.bits {
            false => ELEMENT_A_POS_A::_31_24,
            true => ELEMENT_A_POS_A::_7_0,
        }
    }
    #[doc = "Checks if the value of the field is `_31_24`"]
    #[inline(always)]
    pub fn is_31_24(&self) -> bool {
        *self == ELEMENT_A_POS_A::_31_24
    }
    #[doc = "Checks if the value of the field is `_7_0`"]
    #[inline(always)]
    pub fn is_7_0(&self) -> bool {
        *self == ELEMENT_A_POS_A::_7_0
    }
}
#[doc = "Field `element_a_pos` writer - Element A Position"]
pub type ELEMENT_A_POS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, ELEMENT_A_POS_A, O>;
impl<'a, const O: u8> ELEMENT_A_POS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _31_24(self) -> &'a mut W {
        self.variant(ELEMENT_A_POS_A::_31_24)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _7_0(self) -> &'a mut W {
        self.variant(ELEMENT_A_POS_A::_7_0)
    }
}
#[doc = "Field `rgb_bo` reader - RGB Bit Order"]
pub type RGB_BO_R = crate::BitReader<RGB_BO_A>;
#[doc = "RGB Bit Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGB_BO_A {
    #[doc = "0: `0`"]
    DATA = 0,
    #[doc = "1: `1`"]
    SWAP = 1,
}
impl From<RGB_BO_A> for bool {
    #[inline(always)]
    fn from(variant: RGB_BO_A) -> Self {
        variant as u8 != 0
    }
}
impl RGB_BO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_BO_A {
        match self.bits {
            false => RGB_BO_A::DATA,
            true => RGB_BO_A::SWAP,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RGB_BO_A::DATA
    }
    #[doc = "Checks if the value of the field is `SWAP`"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == RGB_BO_A::SWAP
    }
}
#[doc = "Field `rgb_bo` writer - RGB Bit Order"]
pub type RGB_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, RGB_BO_A, O>;
impl<'a, const O: u8> RGB_BO_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(RGB_BO_A::DATA)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut W {
        self.variant(RGB_BO_A::SWAP)
    }
}
#[doc = "Field `dum_val` reader - Dummy Cycle Value"]
pub type DUM_VAL_R = crate::BitReader<bool>;
#[doc = "Field `dum_val` writer - Dummy Cycle Value"]
pub type DUM_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, bool, O>;
#[doc = "Field `rgb_src_fmt` reader - RGB Source Format"]
pub type RGB_SRC_FMT_R = crate::FieldReader<u8, RGB_SRC_FMT_A>;
#[doc = "RGB Source Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB_SRC_FMT_A {
    #[doc = "0: `0`"]
    RGB = 0,
    #[doc = "1: `1`"]
    RBG = 1,
    #[doc = "2: `10`"]
    GRB = 2,
    #[doc = "3: `11`"]
    GBR = 3,
    #[doc = "4: `100`"]
    BRG = 4,
    #[doc = "5: `101`"]
    BGR = 5,
    #[doc = "6: `110`"]
    GRBG_0 = 6,
    #[doc = "7: `111`"]
    GBRG_0 = 7,
    #[doc = "8: `1000`"]
    GRBG_1 = 8,
    #[doc = "9: `1001`"]
    GBRG_1 = 9,
}
impl From<RGB_SRC_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_SRC_FMT_A) -> Self {
        variant as _
    }
}
impl RGB_SRC_FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGB_SRC_FMT_A> {
        match self.bits {
            0 => Some(RGB_SRC_FMT_A::RGB),
            1 => Some(RGB_SRC_FMT_A::RBG),
            2 => Some(RGB_SRC_FMT_A::GRB),
            3 => Some(RGB_SRC_FMT_A::GBR),
            4 => Some(RGB_SRC_FMT_A::BRG),
            5 => Some(RGB_SRC_FMT_A::BGR),
            6 => Some(RGB_SRC_FMT_A::GRBG_0),
            7 => Some(RGB_SRC_FMT_A::GBRG_0),
            8 => Some(RGB_SRC_FMT_A::GRBG_1),
            9 => Some(RGB_SRC_FMT_A::GBRG_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == RGB_SRC_FMT_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        *self == RGB_SRC_FMT_A::RBG
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        *self == RGB_SRC_FMT_A::GRB
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == RGB_SRC_FMT_A::GBR
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == RGB_SRC_FMT_A::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == RGB_SRC_FMT_A::BGR
    }
    #[doc = "Checks if the value of the field is `GRBG_0`"]
    #[inline(always)]
    pub fn is_grbg_0(&self) -> bool {
        *self == RGB_SRC_FMT_A::GRBG_0
    }
    #[doc = "Checks if the value of the field is `GBRG_0`"]
    #[inline(always)]
    pub fn is_gbrg_0(&self) -> bool {
        *self == RGB_SRC_FMT_A::GBRG_0
    }
    #[doc = "Checks if the value of the field is `GRBG_1`"]
    #[inline(always)]
    pub fn is_grbg_1(&self) -> bool {
        *self == RGB_SRC_FMT_A::GRBG_1
    }
    #[doc = "Checks if the value of the field is `GBRG_1`"]
    #[inline(always)]
    pub fn is_gbrg_1(&self) -> bool {
        *self == RGB_SRC_FMT_A::GBRG_1
    }
}
#[doc = "Field `rgb_src_fmt` writer - RGB Source Format"]
pub type RGB_SRC_FMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CTL_0_SPEC, u8, RGB_SRC_FMT_A, 4, O>;
impl<'a, const O: u8> RGB_SRC_FMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::RGB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::RBG)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GRB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GBR)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::BRG)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::BGR)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn grbg_0(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GRBG_0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn gbrg_0(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GBRG_0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn grbg_1(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GRBG_1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn gbrg_1(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GBRG_1)
    }
}
#[doc = "Field `dbi_interface` reader - "]
pub type DBI_INTERFACE_R = crate::FieldReader<u8, DBI_INTERFACE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBI_INTERFACE_A {
    #[doc = "0: 3 Line Interface I"]
    L3I1 = 0,
    #[doc = "1: 3 Line Interface II"]
    L3I2 = 1,
    #[doc = "2: 4 Line Interface I"]
    L4I1 = 2,
    #[doc = "3: 4 Line Interface II"]
    L4I2 = 3,
    #[doc = "4: 2 Data Lane Interface"]
    D2LI = 4,
}
impl From<DBI_INTERFACE_A> for u8 {
    #[inline(always)]
    fn from(variant: DBI_INTERFACE_A) -> Self {
        variant as _
    }
}
impl DBI_INTERFACE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBI_INTERFACE_A> {
        match self.bits {
            0 => Some(DBI_INTERFACE_A::L3I1),
            1 => Some(DBI_INTERFACE_A::L3I2),
            2 => Some(DBI_INTERFACE_A::L4I1),
            3 => Some(DBI_INTERFACE_A::L4I2),
            4 => Some(DBI_INTERFACE_A::D2LI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `L3I1`"]
    #[inline(always)]
    pub fn is_l3i1(&self) -> bool {
        *self == DBI_INTERFACE_A::L3I1
    }
    #[doc = "Checks if the value of the field is `L3I2`"]
    #[inline(always)]
    pub fn is_l3i2(&self) -> bool {
        *self == DBI_INTERFACE_A::L3I2
    }
    #[doc = "Checks if the value of the field is `L4I1`"]
    #[inline(always)]
    pub fn is_l4i1(&self) -> bool {
        *self == DBI_INTERFACE_A::L4I1
    }
    #[doc = "Checks if the value of the field is `L4I2`"]
    #[inline(always)]
    pub fn is_l4i2(&self) -> bool {
        *self == DBI_INTERFACE_A::L4I2
    }
    #[doc = "Checks if the value of the field is `D2LI`"]
    #[inline(always)]
    pub fn is_d2li(&self) -> bool {
        *self == DBI_INTERFACE_A::D2LI
    }
}
#[doc = "Field `dbi_interface` writer - "]
pub type DBI_INTERFACE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CTL_0_SPEC, u8, DBI_INTERFACE_A, 3, O>;
impl<'a, const O: u8> DBI_INTERFACE_W<'a, O> {
    #[doc = "3 Line Interface I"]
    #[inline(always)]
    pub fn l3i1(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L3I1)
    }
    #[doc = "3 Line Interface II"]
    #[inline(always)]
    pub fn l3i2(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L3I2)
    }
    #[doc = "4 Line Interface I"]
    #[inline(always)]
    pub fn l4i1(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L4I1)
    }
    #[doc = "4 Line Interface II"]
    #[inline(always)]
    pub fn l4i2(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L4I2)
    }
    #[doc = "2 Data Lane Interface"]
    #[inline(always)]
    pub fn d2li(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::D2LI)
    }
}
#[doc = "Field `dat_fmt` reader - Output Data Format"]
pub type DAT_FMT_R = crate::FieldReader<u8, DAT_FMT_A>;
#[doc = "Output Data Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAT_FMT_A {
    #[doc = "0: `0`"]
    RGB111 = 0,
    #[doc = "1: `1`"]
    RGB444 = 1,
    #[doc = "2: `10`"]
    RGB565 = 2,
    #[doc = "3: `11`"]
    RGB666 = 3,
    #[doc = "4: `100`"]
    RGB888 = 4,
}
impl From<DAT_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_FMT_A) -> Self {
        variant as _
    }
}
impl DAT_FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT_FMT_A> {
        match self.bits {
            0 => Some(DAT_FMT_A::RGB111),
            1 => Some(DAT_FMT_A::RGB444),
            2 => Some(DAT_FMT_A::RGB565),
            3 => Some(DAT_FMT_A::RGB666),
            4 => Some(DAT_FMT_A::RGB888),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB111`"]
    #[inline(always)]
    pub fn is_rgb111(&self) -> bool {
        *self == DAT_FMT_A::RGB111
    }
    #[doc = "Checks if the value of the field is `RGB444`"]
    #[inline(always)]
    pub fn is_rgb444(&self) -> bool {
        *self == DAT_FMT_A::RGB444
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == DAT_FMT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB666`"]
    #[inline(always)]
    pub fn is_rgb666(&self) -> bool {
        *self == DAT_FMT_A::RGB666
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == DAT_FMT_A::RGB888
    }
}
#[doc = "Field `dat_fmt` writer - Output Data Format"]
pub type DAT_FMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CTL_0_SPEC, u8, DAT_FMT_A, 3, O>;
impl<'a, const O: u8> DAT_FMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb111(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB111)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB444)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB565)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rgb666(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB666)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB888)
    }
}
#[doc = "Field `tran_mod` reader - Transmit Mode"]
pub type TRAN_MOD_R = crate::BitReader<TRAN_MOD_A>;
#[doc = "Transmit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRAN_MOD_A {
    #[doc = "0: `0`"]
    COMMAND_PARAMETER = 0,
    #[doc = "1: `1`"]
    VIDEO = 1,
}
impl From<TRAN_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: TRAN_MOD_A) -> Self {
        variant as u8 != 0
    }
}
impl TRAN_MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAN_MOD_A {
        match self.bits {
            false => TRAN_MOD_A::COMMAND_PARAMETER,
            true => TRAN_MOD_A::VIDEO,
        }
    }
    #[doc = "Checks if the value of the field is `COMMAND_PARAMETER`"]
    #[inline(always)]
    pub fn is_command_parameter(&self) -> bool {
        *self == TRAN_MOD_A::COMMAND_PARAMETER
    }
    #[doc = "Checks if the value of the field is `VIDEO`"]
    #[inline(always)]
    pub fn is_video(&self) -> bool {
        *self == TRAN_MOD_A::VIDEO
    }
}
#[doc = "Field `tran_mod` writer - Transmit Mode"]
pub type TRAN_MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, TRAN_MOD_A, O>;
impl<'a, const O: u8> TRAN_MOD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn command_parameter(self) -> &'a mut W {
        self.variant(TRAN_MOD_A::COMMAND_PARAMETER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn video(self) -> &'a mut W {
        self.variant(TRAN_MOD_A::VIDEO)
    }
}
#[doc = "Field `rgb_seq` reader - Output RGB Sequence"]
pub type RGB_SEQ_R = crate::FieldReader<u8, RGB_SEQ_A>;
#[doc = "Output RGB Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB_SEQ_A {
    #[doc = "0: `0`"]
    RGB = 0,
    #[doc = "1: `1`"]
    RBG = 1,
    #[doc = "2: `10`"]
    GRB = 2,
    #[doc = "3: `11`"]
    GBR = 3,
    #[doc = "4: `100`"]
    BRG = 4,
    #[doc = "5: `101`"]
    BGR = 5,
}
impl From<RGB_SEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_SEQ_A) -> Self {
        variant as _
    }
}
impl RGB_SEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGB_SEQ_A> {
        match self.bits {
            0 => Some(RGB_SEQ_A::RGB),
            1 => Some(RGB_SEQ_A::RBG),
            2 => Some(RGB_SEQ_A::GRB),
            3 => Some(RGB_SEQ_A::GBR),
            4 => Some(RGB_SEQ_A::BRG),
            5 => Some(RGB_SEQ_A::BGR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == RGB_SEQ_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        *self == RGB_SEQ_A::RBG
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        *self == RGB_SEQ_A::GRB
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == RGB_SEQ_A::GBR
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == RGB_SEQ_A::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == RGB_SEQ_A::BGR
    }
}
#[doc = "Field `rgb_seq` writer - Output RGB Sequence"]
pub type RGB_SEQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CTL_0_SPEC, u8, RGB_SEQ_A, 3, O>;
impl<'a, const O: u8> RGB_SEQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::RGB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::RBG)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::GRB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::GBR)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::BRG)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::BGR)
    }
}
#[doc = "Field `dat_seq` reader - Output Data Sequence"]
pub type DAT_SEQ_R = crate::BitReader<DAT_SEQ_A>;
#[doc = "Output Data Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAT_SEQ_A {
    #[doc = "0: `0`"]
    MSB = 0,
    #[doc = "1: `1`"]
    LSB = 1,
}
impl From<DAT_SEQ_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_SEQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DAT_SEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_SEQ_A {
        match self.bits {
            false => DAT_SEQ_A::MSB,
            true => DAT_SEQ_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == DAT_SEQ_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == DAT_SEQ_A::LSB
    }
}
#[doc = "Field `dat_seq` writer - Output Data Sequence"]
pub type DAT_SEQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, DAT_SEQ_A, O>;
impl<'a, const O: u8> DAT_SEQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DAT_SEQ_A::MSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DAT_SEQ_A::LSB)
    }
}
#[doc = "Field `wcdc` reader - Write Command Dummy Cycles"]
pub type WCDC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wcdc` writer - Write Command Dummy Cycles"]
pub type WCDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBI_CTL_0_SPEC, u16, u16, 11, O>;
#[doc = "Field `cmdt` reader - Command Type"]
pub type CMDT_R = crate::BitReader<CMDT_A>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDT_A {
    #[doc = "0: `0`"]
    WRITE = 0,
    #[doc = "1: `1`"]
    READ = 1,
}
impl From<CMDT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDT_A {
        match self.bits {
            false => CMDT_A::WRITE,
            true => CMDT_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == CMDT_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == CMDT_A::READ
    }
}
#[doc = "Field `cmdt` writer - Command Type"]
pub type CMDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_0_SPEC, CMDT_A, O>;
impl<'a, const O: u8> CMDT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMDT_A::WRITE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMDT_A::READ)
    }
}
impl R {
    #[doc = "Bit 0 - Video Source Type"]
    #[inline(always)]
    pub fn vi_src_type(&self) -> VI_SRC_TYPE_R {
        VI_SRC_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Element A Position"]
    #[inline(always)]
    pub fn element_a_pos(&self) -> ELEMENT_A_POS_R {
        ELEMENT_A_POS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RGB Bit Order"]
    #[inline(always)]
    pub fn rgb_bo(&self) -> RGB_BO_R {
        RGB_BO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dummy Cycle Value"]
    #[inline(always)]
    pub fn dum_val(&self) -> DUM_VAL_R {
        DUM_VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - RGB Source Format"]
    #[inline(always)]
    pub fn rgb_src_fmt(&self) -> RGB_SRC_FMT_R {
        RGB_SRC_FMT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dbi_interface(&self) -> DBI_INTERFACE_R {
        DBI_INTERFACE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Output Data Format"]
    #[inline(always)]
    pub fn dat_fmt(&self) -> DAT_FMT_R {
        DAT_FMT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Transmit Mode"]
    #[inline(always)]
    pub fn tran_mod(&self) -> TRAN_MOD_R {
        TRAN_MOD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Output RGB Sequence"]
    #[inline(always)]
    pub fn rgb_seq(&self) -> RGB_SEQ_R {
        RGB_SEQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Output Data Sequence"]
    #[inline(always)]
    pub fn dat_seq(&self) -> DAT_SEQ_R {
        DAT_SEQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Write Command Dummy Cycles"]
    #[inline(always)]
    pub fn wcdc(&self) -> WCDC_R {
        WCDC_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Command Type"]
    #[inline(always)]
    pub fn cmdt(&self) -> CMDT_R {
        CMDT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Video Source Type"]
    #[inline(always)]
    #[must_use]
    pub fn vi_src_type(&mut self) -> VI_SRC_TYPE_W<0> {
        VI_SRC_TYPE_W::new(self)
    }
    #[doc = "Bit 1 - Element A Position"]
    #[inline(always)]
    #[must_use]
    pub fn element_a_pos(&mut self) -> ELEMENT_A_POS_W<1> {
        ELEMENT_A_POS_W::new(self)
    }
    #[doc = "Bit 2 - RGB Bit Order"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_bo(&mut self) -> RGB_BO_W<2> {
        RGB_BO_W::new(self)
    }
    #[doc = "Bit 3 - Dummy Cycle Value"]
    #[inline(always)]
    #[must_use]
    pub fn dum_val(&mut self) -> DUM_VAL_W<3> {
        DUM_VAL_W::new(self)
    }
    #[doc = "Bits 4:7 - RGB Source Format"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_src_fmt(&mut self) -> RGB_SRC_FMT_W<4> {
        RGB_SRC_FMT_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_interface(&mut self) -> DBI_INTERFACE_W<8> {
        DBI_INTERFACE_W::new(self)
    }
    #[doc = "Bits 12:14 - Output Data Format"]
    #[inline(always)]
    #[must_use]
    pub fn dat_fmt(&mut self) -> DAT_FMT_W<12> {
        DAT_FMT_W::new(self)
    }
    #[doc = "Bit 15 - Transmit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tran_mod(&mut self) -> TRAN_MOD_W<15> {
        TRAN_MOD_W::new(self)
    }
    #[doc = "Bits 16:18 - Output RGB Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_seq(&mut self) -> RGB_SEQ_W<16> {
        RGB_SEQ_W::new(self)
    }
    #[doc = "Bit 19 - Output Data Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn dat_seq(&mut self) -> DAT_SEQ_W<19> {
        DAT_SEQ_W::new(self)
    }
    #[doc = "Bits 20:30 - Write Command Dummy Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn wcdc(&mut self) -> WCDC_W<20> {
        WCDC_W::new(self)
    }
    #[doc = "Bit 31 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdt(&mut self) -> CMDT_W<31> {
        CMDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_ctl_0](index.html) module"]
pub struct DBI_CTL_0_SPEC;
impl crate::RegisterSpec for DBI_CTL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_ctl_0::R](R) reader structure"]
impl crate::Readable for DBI_CTL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_ctl_0::W](W) writer structure"]
impl crate::Writable for DBI_CTL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbi_ctl_0 to value 0"]
impl crate::Resettable for DBI_CTL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
