#[doc = "Register `pd_drv2` reader"]
pub struct R(crate::R<PD_DRV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_DRV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_DRV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_DRV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_drv2` writer"]
pub struct W(crate::W<PD_DRV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_DRV2_SPEC>;
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
impl From<crate::W<PD_DRV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_DRV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD22_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD22_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD22_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd22_drv` reader - "]
pub struct PD22_DRV_R(crate::FieldReader<u8, PD22_DRV_A>);
impl PD22_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD22_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD22_DRV_A {
        match self.bits {
            0 => PD22_DRV_A::L0,
            1 => PD22_DRV_A::L1,
            2 => PD22_DRV_A::L2,
            3 => PD22_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD22_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD22_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD22_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD22_DRV_A::L3
    }
}
impl core::ops::Deref for PD22_DRV_R {
    type Target = crate::FieldReader<u8, PD22_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd22_drv` writer - "]
pub struct PD22_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD22_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD22_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD22_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD22_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD22_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD22_DRV_A::L3)
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
pub enum PD21_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD21_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD21_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd21_drv` reader - "]
pub struct PD21_DRV_R(crate::FieldReader<u8, PD21_DRV_A>);
impl PD21_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD21_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD21_DRV_A {
        match self.bits {
            0 => PD21_DRV_A::L0,
            1 => PD21_DRV_A::L1,
            2 => PD21_DRV_A::L2,
            3 => PD21_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD21_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD21_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD21_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD21_DRV_A::L3
    }
}
impl core::ops::Deref for PD21_DRV_R {
    type Target = crate::FieldReader<u8, PD21_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd21_drv` writer - "]
pub struct PD21_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD21_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD21_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD21_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD21_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD21_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD21_DRV_A::L3)
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
pub enum PD20_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD20_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD20_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd20_drv` reader - "]
pub struct PD20_DRV_R(crate::FieldReader<u8, PD20_DRV_A>);
impl PD20_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD20_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD20_DRV_A {
        match self.bits {
            0 => PD20_DRV_A::L0,
            1 => PD20_DRV_A::L1,
            2 => PD20_DRV_A::L2,
            3 => PD20_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD20_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD20_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD20_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD20_DRV_A::L3
    }
}
impl core::ops::Deref for PD20_DRV_R {
    type Target = crate::FieldReader<u8, PD20_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd20_drv` writer - "]
pub struct PD20_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD20_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD20_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD20_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD20_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD20_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD20_DRV_A::L3)
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
pub enum PD19_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD19_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD19_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd19_drv` reader - "]
pub struct PD19_DRV_R(crate::FieldReader<u8, PD19_DRV_A>);
impl PD19_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD19_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD19_DRV_A {
        match self.bits {
            0 => PD19_DRV_A::L0,
            1 => PD19_DRV_A::L1,
            2 => PD19_DRV_A::L2,
            3 => PD19_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD19_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD19_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD19_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD19_DRV_A::L3
    }
}
impl core::ops::Deref for PD19_DRV_R {
    type Target = crate::FieldReader<u8, PD19_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd19_drv` writer - "]
pub struct PD19_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD19_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD19_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD19_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD19_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD19_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD19_DRV_A::L3)
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
pub enum PD18_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD18_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD18_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd18_drv` reader - "]
pub struct PD18_DRV_R(crate::FieldReader<u8, PD18_DRV_A>);
impl PD18_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD18_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD18_DRV_A {
        match self.bits {
            0 => PD18_DRV_A::L0,
            1 => PD18_DRV_A::L1,
            2 => PD18_DRV_A::L2,
            3 => PD18_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD18_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD18_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD18_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD18_DRV_A::L3
    }
}
impl core::ops::Deref for PD18_DRV_R {
    type Target = crate::FieldReader<u8, PD18_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd18_drv` writer - "]
pub struct PD18_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD18_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD18_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD18_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD18_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD18_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD18_DRV_A::L3)
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
pub enum PD17_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD17_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD17_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd17_drv` reader - "]
pub struct PD17_DRV_R(crate::FieldReader<u8, PD17_DRV_A>);
impl PD17_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD17_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD17_DRV_A {
        match self.bits {
            0 => PD17_DRV_A::L0,
            1 => PD17_DRV_A::L1,
            2 => PD17_DRV_A::L2,
            3 => PD17_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD17_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD17_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD17_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD17_DRV_A::L3
    }
}
impl core::ops::Deref for PD17_DRV_R {
    type Target = crate::FieldReader<u8, PD17_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd17_drv` writer - "]
pub struct PD17_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD17_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD17_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD17_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD17_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD17_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD17_DRV_A::L3)
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
pub enum PD16_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PD16_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PD16_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd16_drv` reader - "]
pub struct PD16_DRV_R(crate::FieldReader<u8, PD16_DRV_A>);
impl PD16_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD16_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD16_DRV_A {
        match self.bits {
            0 => PD16_DRV_A::L0,
            1 => PD16_DRV_A::L1,
            2 => PD16_DRV_A::L2,
            3 => PD16_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PD16_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PD16_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PD16_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PD16_DRV_A::L3
    }
}
impl core::ops::Deref for PD16_DRV_R {
    type Target = crate::FieldReader<u8, PD16_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd16_drv` writer - "]
pub struct PD16_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD16_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD16_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PD16_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PD16_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PD16_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PD16_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd22_drv(&self) -> PD22_DRV_R {
        PD22_DRV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd21_drv(&self) -> PD21_DRV_R {
        PD21_DRV_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd20_drv(&self) -> PD20_DRV_R {
        PD20_DRV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd19_drv(&self) -> PD19_DRV_R {
        PD19_DRV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd18_drv(&self) -> PD18_DRV_R {
        PD18_DRV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd17_drv(&self) -> PD17_DRV_R {
        PD17_DRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd16_drv(&self) -> PD16_DRV_R {
        PD16_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd22_drv(&mut self) -> PD22_DRV_W {
        PD22_DRV_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd21_drv(&mut self) -> PD21_DRV_W {
        PD21_DRV_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd20_drv(&mut self) -> PD20_DRV_W {
        PD20_DRV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd19_drv(&mut self) -> PD19_DRV_W {
        PD19_DRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd18_drv(&mut self) -> PD18_DRV_W {
        PD18_DRV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd17_drv(&mut self) -> PD17_DRV_W {
        PD17_DRV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd16_drv(&mut self) -> PD16_DRV_W {
        PD16_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Multi_Driving Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_drv2](index.html) module"]
pub struct PD_DRV2_SPEC;
impl crate::RegisterSpec for PD_DRV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_drv2::R](R) reader structure"]
impl crate::Readable for PD_DRV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_drv2::W](W) writer structure"]
impl crate::Writable for PD_DRV2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_drv2 to value 0"]
impl crate::Resettable for PD_DRV2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
