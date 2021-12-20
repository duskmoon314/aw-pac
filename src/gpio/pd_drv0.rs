#[doc = "Register `pd_drv0` reader"]
pub struct R(crate::R<PD_DRV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_DRV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_DRV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_DRV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_drv0` writer"]
pub struct W(crate::W<PD_DRV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_DRV0_SPEC>;
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
impl From<crate::W<PD_DRV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_DRV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD7_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD7_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD7_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd7_drv` reader - "]
pub struct PD7_DRV_R(crate::FieldReader<u8, PD7_DRV_A>);
impl PD7_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD7_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD7_DRV_A {
        match self.bits {
            0 => PD7_DRV_A::L0,
            1 => PD7_DRV_A::L1,
            2 => PD7_DRV_A::L2,
            3 => PD7_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD7_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD7_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD7_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD7_DRV_A::L3
    }
}
impl core::ops::Deref for PD7_DRV_R {
    type Target = crate::FieldReader<u8, PD7_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd7_drv` writer - "]
pub struct PD7_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD7_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD7_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD7_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD7_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD7_DRV_A::L3)
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
pub enum PD6_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD6_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD6_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd6_drv` reader - "]
pub struct PD6_DRV_R(crate::FieldReader<u8, PD6_DRV_A>);
impl PD6_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD6_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD6_DRV_A {
        match self.bits {
            0 => PD6_DRV_A::L0,
            1 => PD6_DRV_A::L1,
            2 => PD6_DRV_A::L2,
            3 => PD6_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD6_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD6_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD6_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD6_DRV_A::L3
    }
}
impl core::ops::Deref for PD6_DRV_R {
    type Target = crate::FieldReader<u8, PD6_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd6_drv` writer - "]
pub struct PD6_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD6_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD6_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD6_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD6_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD6_DRV_A::L3)
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
pub enum PD5_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD5_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD5_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd5_drv` reader - "]
pub struct PD5_DRV_R(crate::FieldReader<u8, PD5_DRV_A>);
impl PD5_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD5_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD5_DRV_A {
        match self.bits {
            0 => PD5_DRV_A::L0,
            1 => PD5_DRV_A::L1,
            2 => PD5_DRV_A::L2,
            3 => PD5_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD5_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD5_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD5_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD5_DRV_A::L3
    }
}
impl core::ops::Deref for PD5_DRV_R {
    type Target = crate::FieldReader<u8, PD5_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd5_drv` writer - "]
pub struct PD5_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD5_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD5_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD5_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD5_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD5_DRV_A::L3)
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
pub enum PD4_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD4_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD4_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd4_drv` reader - "]
pub struct PD4_DRV_R(crate::FieldReader<u8, PD4_DRV_A>);
impl PD4_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD4_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD4_DRV_A {
        match self.bits {
            0 => PD4_DRV_A::L0,
            1 => PD4_DRV_A::L1,
            2 => PD4_DRV_A::L2,
            3 => PD4_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD4_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD4_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD4_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD4_DRV_A::L3
    }
}
impl core::ops::Deref for PD4_DRV_R {
    type Target = crate::FieldReader<u8, PD4_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd4_drv` writer - "]
pub struct PD4_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD4_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD4_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD4_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD4_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD4_DRV_A::L3)
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
pub enum PD3_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD3_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD3_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd3_drv` reader - "]
pub struct PD3_DRV_R(crate::FieldReader<u8, PD3_DRV_A>);
impl PD3_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD3_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD3_DRV_A {
        match self.bits {
            0 => PD3_DRV_A::L0,
            1 => PD3_DRV_A::L1,
            2 => PD3_DRV_A::L2,
            3 => PD3_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD3_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD3_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD3_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD3_DRV_A::L3
    }
}
impl core::ops::Deref for PD3_DRV_R {
    type Target = crate::FieldReader<u8, PD3_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd3_drv` writer - "]
pub struct PD3_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD3_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD3_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD3_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD3_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD3_DRV_A::L3)
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
pub enum PD2_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD2_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD2_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd2_drv` reader - "]
pub struct PD2_DRV_R(crate::FieldReader<u8, PD2_DRV_A>);
impl PD2_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD2_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD2_DRV_A {
        match self.bits {
            0 => PD2_DRV_A::L0,
            1 => PD2_DRV_A::L1,
            2 => PD2_DRV_A::L2,
            3 => PD2_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD2_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD2_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD2_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD2_DRV_A::L3
    }
}
impl core::ops::Deref for PD2_DRV_R {
    type Target = crate::FieldReader<u8, PD2_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd2_drv` writer - "]
pub struct PD2_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD2_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD2_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD2_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD2_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD2_DRV_A::L3)
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
pub enum PD1_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD1_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD1_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd1_drv` reader - "]
pub struct PD1_DRV_R(crate::FieldReader<u8, PD1_DRV_A>);
impl PD1_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD1_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD1_DRV_A {
        match self.bits {
            0 => PD1_DRV_A::L0,
            1 => PD1_DRV_A::L1,
            2 => PD1_DRV_A::L2,
            3 => PD1_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD1_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD1_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD1_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD1_DRV_A::L3
    }
}
impl core::ops::Deref for PD1_DRV_R {
    type Target = crate::FieldReader<u8, PD1_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd1_drv` writer - "]
pub struct PD1_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD1_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD1_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD1_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD1_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD1_DRV_A::L3)
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
pub enum PD0_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD0_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD0_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd0_drv` reader - "]
pub struct PD0_DRV_R(crate::FieldReader<u8, PD0_DRV_A>);
impl PD0_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD0_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD0_DRV_A {
        match self.bits {
            0 => PD0_DRV_A::L0,
            1 => PD0_DRV_A::L1,
            2 => PD0_DRV_A::L2,
            3 => PD0_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD0_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD0_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD0_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD0_DRV_A::L3
    }
}
impl core::ops::Deref for PD0_DRV_R {
    type Target = crate::FieldReader<u8, PD0_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd0_drv` writer - "]
pub struct PD0_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD0_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD0_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD0_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD0_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD0_DRV_A::L3)
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
    pub fn pd7_drv(&self) -> PD7_DRV_R {
        PD7_DRV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd6_drv(&self) -> PD6_DRV_R {
        PD6_DRV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd5_drv(&self) -> PD5_DRV_R {
        PD5_DRV_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd4_drv(&self) -> PD4_DRV_R {
        PD4_DRV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd3_drv(&self) -> PD3_DRV_R {
        PD3_DRV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd2_drv(&self) -> PD2_DRV_R {
        PD2_DRV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd1_drv(&self) -> PD1_DRV_R {
        PD1_DRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd0_drv(&self) -> PD0_DRV_R {
        PD0_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pd7_drv(&mut self) -> PD7_DRV_W {
        PD7_DRV_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd6_drv(&mut self) -> PD6_DRV_W {
        PD6_DRV_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd5_drv(&mut self) -> PD5_DRV_W {
        PD5_DRV_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd4_drv(&mut self) -> PD4_DRV_W {
        PD4_DRV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd3_drv(&mut self) -> PD3_DRV_W {
        PD3_DRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd2_drv(&mut self) -> PD2_DRV_W {
        PD2_DRV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd1_drv(&mut self) -> PD1_DRV_W {
        PD1_DRV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd0_drv(&mut self) -> PD0_DRV_W {
        PD0_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Multi_Driving Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_drv0](index.html) module"]
pub struct PD_DRV0_SPEC;
impl crate::RegisterSpec for PD_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_drv0::R](R) reader structure"]
impl crate::Readable for PD_DRV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_drv0::W](W) writer structure"]
impl crate::Writable for PD_DRV0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_drv0 to value 0"]
impl crate::Resettable for PD_DRV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
