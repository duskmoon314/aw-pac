#[doc = "Register `pd_drv1` reader"]
pub struct R(crate::R<PD_DRV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_DRV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_DRV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_DRV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_drv1` writer"]
pub struct W(crate::W<PD_DRV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_DRV1_SPEC>;
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
impl From<crate::W<PD_DRV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_DRV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD15_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD15_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD15_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd15_drv` reader - "]
pub struct PD15_DRV_R(crate::FieldReader<u8, PD15_DRV_A>);
impl PD15_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD15_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD15_DRV_A {
        match self.bits {
            0 => PD15_DRV_A::L0,
            1 => PD15_DRV_A::L1,
            2 => PD15_DRV_A::L2,
            3 => PD15_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD15_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD15_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD15_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD15_DRV_A::L3
    }
}
impl core::ops::Deref for PD15_DRV_R {
    type Target = crate::FieldReader<u8, PD15_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd15_drv` writer - "]
pub struct PD15_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD15_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD15_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD15_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD15_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD15_DRV_A::L3)
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
pub enum PD14_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD14_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD14_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd14_drv` reader - "]
pub struct PD14_DRV_R(crate::FieldReader<u8, PD14_DRV_A>);
impl PD14_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD14_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD14_DRV_A {
        match self.bits {
            0 => PD14_DRV_A::L0,
            1 => PD14_DRV_A::L1,
            2 => PD14_DRV_A::L2,
            3 => PD14_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD14_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD14_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD14_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD14_DRV_A::L3
    }
}
impl core::ops::Deref for PD14_DRV_R {
    type Target = crate::FieldReader<u8, PD14_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd14_drv` writer - "]
pub struct PD14_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD14_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD14_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD14_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD14_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD14_DRV_A::L3)
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
pub enum PD13_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD13_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD13_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd13_drv` reader - "]
pub struct PD13_DRV_R(crate::FieldReader<u8, PD13_DRV_A>);
impl PD13_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD13_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD13_DRV_A {
        match self.bits {
            0 => PD13_DRV_A::L0,
            1 => PD13_DRV_A::L1,
            2 => PD13_DRV_A::L2,
            3 => PD13_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD13_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD13_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD13_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD13_DRV_A::L3
    }
}
impl core::ops::Deref for PD13_DRV_R {
    type Target = crate::FieldReader<u8, PD13_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd13_drv` writer - "]
pub struct PD13_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD13_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD13_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD13_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD13_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD13_DRV_A::L3)
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
pub enum PD12_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD12_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD12_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd12_drv` reader - "]
pub struct PD12_DRV_R(crate::FieldReader<u8, PD12_DRV_A>);
impl PD12_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD12_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD12_DRV_A {
        match self.bits {
            0 => PD12_DRV_A::L0,
            1 => PD12_DRV_A::L1,
            2 => PD12_DRV_A::L2,
            3 => PD12_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD12_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD12_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD12_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD12_DRV_A::L3
    }
}
impl core::ops::Deref for PD12_DRV_R {
    type Target = crate::FieldReader<u8, PD12_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd12_drv` writer - "]
pub struct PD12_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD12_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD12_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD12_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD12_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD12_DRV_A::L3)
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
pub enum PD11_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD11_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD11_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd11_drv` reader - "]
pub struct PD11_DRV_R(crate::FieldReader<u8, PD11_DRV_A>);
impl PD11_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD11_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD11_DRV_A {
        match self.bits {
            0 => PD11_DRV_A::L0,
            1 => PD11_DRV_A::L1,
            2 => PD11_DRV_A::L2,
            3 => PD11_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD11_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD11_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD11_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD11_DRV_A::L3
    }
}
impl core::ops::Deref for PD11_DRV_R {
    type Target = crate::FieldReader<u8, PD11_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd11_drv` writer - "]
pub struct PD11_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD11_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD11_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD11_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD11_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD11_DRV_A::L3)
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
pub enum PD10_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD10_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD10_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd10_drv` reader - "]
pub struct PD10_DRV_R(crate::FieldReader<u8, PD10_DRV_A>);
impl PD10_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD10_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD10_DRV_A {
        match self.bits {
            0 => PD10_DRV_A::L0,
            1 => PD10_DRV_A::L1,
            2 => PD10_DRV_A::L2,
            3 => PD10_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD10_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD10_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD10_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD10_DRV_A::L3
    }
}
impl core::ops::Deref for PD10_DRV_R {
    type Target = crate::FieldReader<u8, PD10_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd10_drv` writer - "]
pub struct PD10_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD10_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD10_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD10_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD10_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD10_DRV_A::L3)
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
pub enum PD9_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD9_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD9_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd9_drv` reader - "]
pub struct PD9_DRV_R(crate::FieldReader<u8, PD9_DRV_A>);
impl PD9_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD9_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD9_DRV_A {
        match self.bits {
            0 => PD9_DRV_A::L0,
            1 => PD9_DRV_A::L1,
            2 => PD9_DRV_A::L2,
            3 => PD9_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD9_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD9_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD9_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD9_DRV_A::L3
    }
}
impl core::ops::Deref for PD9_DRV_R {
    type Target = crate::FieldReader<u8, PD9_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd9_drv` writer - "]
pub struct PD9_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD9_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD9_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD9_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD9_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD9_DRV_A::L3)
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
pub enum PD8_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD8_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD8_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd8_drv` reader - "]
pub struct PD8_DRV_R(crate::FieldReader<u8, PD8_DRV_A>);
impl PD8_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD8_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD8_DRV_A {
        match self.bits {
            0 => PD8_DRV_A::L0,
            1 => PD8_DRV_A::L1,
            2 => PD8_DRV_A::L2,
            3 => PD8_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD8_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD8_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD8_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD8_DRV_A::L3
    }
}
impl core::ops::Deref for PD8_DRV_R {
    type Target = crate::FieldReader<u8, PD8_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd8_drv` writer - "]
pub struct PD8_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD8_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD8_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD8_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD8_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD8_DRV_A::L3)
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
    pub fn pd15_drv(&self) -> PD15_DRV_R {
        PD15_DRV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd14_drv(&self) -> PD14_DRV_R {
        PD14_DRV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd13_drv(&self) -> PD13_DRV_R {
        PD13_DRV_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd12_drv(&self) -> PD12_DRV_R {
        PD12_DRV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd11_drv(&self) -> PD11_DRV_R {
        PD11_DRV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd10_drv(&self) -> PD10_DRV_R {
        PD10_DRV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd9_drv(&self) -> PD9_DRV_R {
        PD9_DRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd8_drv(&self) -> PD8_DRV_R {
        PD8_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pd15_drv(&mut self) -> PD15_DRV_W {
        PD15_DRV_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd14_drv(&mut self) -> PD14_DRV_W {
        PD14_DRV_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd13_drv(&mut self) -> PD13_DRV_W {
        PD13_DRV_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd12_drv(&mut self) -> PD12_DRV_W {
        PD12_DRV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd11_drv(&mut self) -> PD11_DRV_W {
        PD11_DRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd10_drv(&mut self) -> PD10_DRV_W {
        PD10_DRV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd9_drv(&mut self) -> PD9_DRV_W {
        PD9_DRV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd8_drv(&mut self) -> PD8_DRV_W {
        PD8_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Multi_Driving Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_drv1](index.html) module"]
pub struct PD_DRV1_SPEC;
impl crate::RegisterSpec for PD_DRV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_drv1::R](R) reader structure"]
impl crate::Readable for PD_DRV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_drv1::W](W) writer structure"]
impl crate::Writable for PD_DRV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_drv1 to value 0"]
impl crate::Resettable for PD_DRV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
