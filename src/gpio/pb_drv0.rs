#[doc = "Register `pb_drv0` reader"]
pub struct R(crate::R<PB_DRV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_DRV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_DRV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_DRV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pb_drv0` writer"]
pub struct W(crate::W<PB_DRV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_DRV0_SPEC>;
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
impl From<crate::W<PB_DRV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_DRV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB7_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB7_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB7_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb7_drv` reader - "]
pub struct PB7_DRV_R(crate::FieldReader<u8, PB7_DRV_A>);
impl PB7_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB7_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB7_DRV_A {
        match self.bits {
            0 => PB7_DRV_A::L0,
            1 => PB7_DRV_A::L1,
            2 => PB7_DRV_A::L2,
            3 => PB7_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB7_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB7_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB7_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB7_DRV_A::L3
    }
}
impl core::ops::Deref for PB7_DRV_R {
    type Target = crate::FieldReader<u8, PB7_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb7_drv` writer - "]
pub struct PB7_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB7_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB7_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB7_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB7_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB7_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB6_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB6_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB6_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb6_drv` reader - "]
pub struct PB6_DRV_R(crate::FieldReader<u8, PB6_DRV_A>);
impl PB6_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB6_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB6_DRV_A {
        match self.bits {
            0 => PB6_DRV_A::L0,
            1 => PB6_DRV_A::L1,
            2 => PB6_DRV_A::L2,
            3 => PB6_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB6_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB6_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB6_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB6_DRV_A::L3
    }
}
impl core::ops::Deref for PB6_DRV_R {
    type Target = crate::FieldReader<u8, PB6_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb6_drv` writer - "]
pub struct PB6_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB6_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB6_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB6_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB6_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB6_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB5_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB5_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB5_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb5_drv` reader - "]
pub struct PB5_DRV_R(crate::FieldReader<u8, PB5_DRV_A>);
impl PB5_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB5_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB5_DRV_A {
        match self.bits {
            0 => PB5_DRV_A::L0,
            1 => PB5_DRV_A::L1,
            2 => PB5_DRV_A::L2,
            3 => PB5_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB5_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB5_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB5_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB5_DRV_A::L3
    }
}
impl core::ops::Deref for PB5_DRV_R {
    type Target = crate::FieldReader<u8, PB5_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb5_drv` writer - "]
pub struct PB5_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB5_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB5_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB5_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB5_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB5_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB5_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB4_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB4_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB4_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb4_drv` reader - "]
pub struct PB4_DRV_R(crate::FieldReader<u8, PB4_DRV_A>);
impl PB4_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB4_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB4_DRV_A {
        match self.bits {
            0 => PB4_DRV_A::L0,
            1 => PB4_DRV_A::L1,
            2 => PB4_DRV_A::L2,
            3 => PB4_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB4_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB4_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB4_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB4_DRV_A::L3
    }
}
impl core::ops::Deref for PB4_DRV_R {
    type Target = crate::FieldReader<u8, PB4_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb4_drv` writer - "]
pub struct PB4_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB4_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB4_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB4_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB4_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB4_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB4_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB3_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB3_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB3_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb3_drv` reader - "]
pub struct PB3_DRV_R(crate::FieldReader<u8, PB3_DRV_A>);
impl PB3_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB3_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB3_DRV_A {
        match self.bits {
            0 => PB3_DRV_A::L0,
            1 => PB3_DRV_A::L1,
            2 => PB3_DRV_A::L2,
            3 => PB3_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB3_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB3_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB3_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB3_DRV_A::L3
    }
}
impl core::ops::Deref for PB3_DRV_R {
    type Target = crate::FieldReader<u8, PB3_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb3_drv` writer - "]
pub struct PB3_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB3_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB3_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB3_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB3_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB3_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB3_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB2_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB2_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB2_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb2_drv` reader - "]
pub struct PB2_DRV_R(crate::FieldReader<u8, PB2_DRV_A>);
impl PB2_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB2_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB2_DRV_A {
        match self.bits {
            0 => PB2_DRV_A::L0,
            1 => PB2_DRV_A::L1,
            2 => PB2_DRV_A::L2,
            3 => PB2_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB2_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB2_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB2_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB2_DRV_A::L3
    }
}
impl core::ops::Deref for PB2_DRV_R {
    type Target = crate::FieldReader<u8, PB2_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb2_drv` writer - "]
pub struct PB2_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB2_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB2_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB2_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB2_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB2_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB2_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB1_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB1_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB1_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb1_drv` reader - "]
pub struct PB1_DRV_R(crate::FieldReader<u8, PB1_DRV_A>);
impl PB1_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB1_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB1_DRV_A {
        match self.bits {
            0 => PB1_DRV_A::L0,
            1 => PB1_DRV_A::L1,
            2 => PB1_DRV_A::L2,
            3 => PB1_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB1_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB1_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB1_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB1_DRV_A::L3
    }
}
impl core::ops::Deref for PB1_DRV_R {
    type Target = crate::FieldReader<u8, PB1_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb1_drv` writer - "]
pub struct PB1_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB1_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB1_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB1_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB1_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB1_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB1_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB0_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB0_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB0_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb0_drv` reader - "]
pub struct PB0_DRV_R(crate::FieldReader<u8, PB0_DRV_A>);
impl PB0_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB0_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB0_DRV_A {
        match self.bits {
            0 => PB0_DRV_A::L0,
            1 => PB0_DRV_A::L1,
            2 => PB0_DRV_A::L2,
            3 => PB0_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB0_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB0_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB0_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB0_DRV_A::L3
    }
}
impl core::ops::Deref for PB0_DRV_R {
    type Target = crate::FieldReader<u8, PB0_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb0_drv` writer - "]
pub struct PB0_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB0_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB0_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB0_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB0_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB0_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB0_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pb7_drv(&self) -> PB7_DRV_R {
        PB7_DRV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pb6_drv(&self) -> PB6_DRV_R {
        PB6_DRV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pb5_drv(&self) -> PB5_DRV_R {
        PB5_DRV_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pb4_drv(&self) -> PB4_DRV_R {
        PB4_DRV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pb3_drv(&self) -> PB3_DRV_R {
        PB3_DRV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pb2_drv(&self) -> PB2_DRV_R {
        PB2_DRV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pb1_drv(&self) -> PB1_DRV_R {
        PB1_DRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pb0_drv(&self) -> PB0_DRV_R {
        PB0_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pb7_drv(&mut self) -> PB7_DRV_W {
        PB7_DRV_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pb6_drv(&mut self) -> PB6_DRV_W {
        PB6_DRV_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pb5_drv(&mut self) -> PB5_DRV_W {
        PB5_DRV_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pb4_drv(&mut self) -> PB4_DRV_W {
        PB4_DRV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pb3_drv(&mut self) -> PB3_DRV_W {
        PB3_DRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pb2_drv(&mut self) -> PB2_DRV_W {
        PB2_DRV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pb1_drv(&mut self) -> PB1_DRV_W {
        PB1_DRV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pb0_drv(&mut self) -> PB0_DRV_W {
        PB0_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB Multi_Driving Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_drv0](index.html) module"]
pub struct PB_DRV0_SPEC;
impl crate::RegisterSpec for PB_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_drv0::R](R) reader structure"]
impl crate::Readable for PB_DRV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_drv0::W](W) writer structure"]
impl crate::Writable for PB_DRV0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pb_drv0 to value 0"]
impl crate::Resettable for PB_DRV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
