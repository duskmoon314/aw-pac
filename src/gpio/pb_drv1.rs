#[doc = "Register `pb_drv1` reader"]
pub struct R(crate::R<PB_DRV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_DRV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_DRV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_DRV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pb_drv1` writer"]
pub struct W(crate::W<PB_DRV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_DRV1_SPEC>;
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
impl From<crate::W<PB_DRV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_DRV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB12_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB12_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB12_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb12_drv` reader - "]
pub struct PB12_DRV_R(crate::FieldReader<u8, PB12_DRV_A>);
impl PB12_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB12_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB12_DRV_A {
        match self.bits {
            0 => PB12_DRV_A::L0,
            1 => PB12_DRV_A::L1,
            2 => PB12_DRV_A::L2,
            3 => PB12_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB12_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB12_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB12_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB12_DRV_A::L3
    }
}
impl core::ops::Deref for PB12_DRV_R {
    type Target = crate::FieldReader<u8, PB12_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb12_drv` writer - "]
pub struct PB12_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB12_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB12_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB12_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB12_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB12_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB12_DRV_A::L3)
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
pub enum PB11_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB11_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB11_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb11_drv` reader - "]
pub struct PB11_DRV_R(crate::FieldReader<u8, PB11_DRV_A>);
impl PB11_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB11_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB11_DRV_A {
        match self.bits {
            0 => PB11_DRV_A::L0,
            1 => PB11_DRV_A::L1,
            2 => PB11_DRV_A::L2,
            3 => PB11_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB11_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB11_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB11_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB11_DRV_A::L3
    }
}
impl core::ops::Deref for PB11_DRV_R {
    type Target = crate::FieldReader<u8, PB11_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb11_drv` writer - "]
pub struct PB11_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB11_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB11_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB11_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB11_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB11_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB11_DRV_A::L3)
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
pub enum PB10_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB10_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB10_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb10_drv` reader - "]
pub struct PB10_DRV_R(crate::FieldReader<u8, PB10_DRV_A>);
impl PB10_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB10_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB10_DRV_A {
        match self.bits {
            0 => PB10_DRV_A::L0,
            1 => PB10_DRV_A::L1,
            2 => PB10_DRV_A::L2,
            3 => PB10_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB10_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB10_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB10_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB10_DRV_A::L3
    }
}
impl core::ops::Deref for PB10_DRV_R {
    type Target = crate::FieldReader<u8, PB10_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb10_drv` writer - "]
pub struct PB10_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB10_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB10_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB10_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB10_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB10_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB10_DRV_A::L3)
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
pub enum PB9_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB9_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB9_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb9_drv` reader - "]
pub struct PB9_DRV_R(crate::FieldReader<u8, PB9_DRV_A>);
impl PB9_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB9_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB9_DRV_A {
        match self.bits {
            0 => PB9_DRV_A::L0,
            1 => PB9_DRV_A::L1,
            2 => PB9_DRV_A::L2,
            3 => PB9_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB9_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB9_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB9_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB9_DRV_A::L3
    }
}
impl core::ops::Deref for PB9_DRV_R {
    type Target = crate::FieldReader<u8, PB9_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb9_drv` writer - "]
pub struct PB9_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB9_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB9_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB9_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB9_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB9_DRV_A::L3)
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
pub enum PB8_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB8_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB8_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb8_drv` reader - "]
pub struct PB8_DRV_R(crate::FieldReader<u8, PB8_DRV_A>);
impl PB8_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB8_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB8_DRV_A {
        match self.bits {
            0 => PB8_DRV_A::L0,
            1 => PB8_DRV_A::L1,
            2 => PB8_DRV_A::L2,
            3 => PB8_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PB8_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PB8_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PB8_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PB8_DRV_A::L3
    }
}
impl core::ops::Deref for PB8_DRV_R {
    type Target = crate::FieldReader<u8, PB8_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb8_drv` writer - "]
pub struct PB8_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB8_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PB8_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PB8_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PB8_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PB8_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pb12_drv(&self) -> PB12_DRV_R {
        PB12_DRV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pb11_drv(&self) -> PB11_DRV_R {
        PB11_DRV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pb10_drv(&self) -> PB10_DRV_R {
        PB10_DRV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pb9_drv(&self) -> PB9_DRV_R {
        PB9_DRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pb8_drv(&self) -> PB8_DRV_R {
        PB8_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pb12_drv(&mut self) -> PB12_DRV_W {
        PB12_DRV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pb11_drv(&mut self) -> PB11_DRV_W {
        PB11_DRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pb10_drv(&mut self) -> PB10_DRV_W {
        PB10_DRV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pb9_drv(&mut self) -> PB9_DRV_W {
        PB9_DRV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pb8_drv(&mut self) -> PB8_DRV_W {
        PB8_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB Multi_Driving Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_drv1](index.html) module"]
pub struct PB_DRV1_SPEC;
impl crate::RegisterSpec for PB_DRV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_drv1::R](R) reader structure"]
impl crate::Readable for PB_DRV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_drv1::W](W) writer structure"]
impl crate::Writable for PB_DRV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pb_drv1 to value 0"]
impl crate::Resettable for PB_DRV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
