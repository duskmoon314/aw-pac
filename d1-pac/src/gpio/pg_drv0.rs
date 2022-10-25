#[doc = "Register `pg_drv0` reader"]
pub struct R(crate::R<PG_DRV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_DRV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_DRV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_DRV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_drv0` writer"]
pub struct W(crate::W<PG_DRV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_DRV0_SPEC>;
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
impl From<crate::W<PG_DRV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_DRV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pg_drv[0-7]` reader - PG Multi_Driving Select"]
pub type PG_DRV_R = crate::FieldReader<u8, PG_DRV_A>;
#[doc = "PG Multi_Driving Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PG_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PG_DRV_A) -> Self {
        variant as _
    }
}
impl PG_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PG_DRV_A {
        match self.bits {
            0 => PG_DRV_A::L0,
            1 => PG_DRV_A::L1,
            2 => PG_DRV_A::L2,
            3 => PG_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        *self == PG_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        *self == PG_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        *self == PG_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        *self == PG_DRV_A::L3
    }
}
#[doc = "Field `pg_drv[0-7]` writer - PG Multi_Driving Select"]
pub type PG_DRV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PG_DRV0_SPEC, u8, PG_DRV_A, 2, O>;
impl<'a, const O: u8> PG_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PG_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PG_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PG_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PG_DRV_A::L3)
    }
}
impl R {
    #[doc = "PG Multi_Driving Select"]
    #[inline(always)]
    pub unsafe fn pg_drv(&self, n: u8) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg0_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg1_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg2_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg3_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg4_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg5_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg6_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg7_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pg_drv<const O: u8>(&mut self) -> PG_DRV_W<O> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 0:1 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg0_drv(&mut self) -> PG_DRV_W<0> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 4:5 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg1_drv(&mut self) -> PG_DRV_W<4> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 8:9 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg2_drv(&mut self) -> PG_DRV_W<8> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 12:13 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg3_drv(&mut self) -> PG_DRV_W<12> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 16:17 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg4_drv(&mut self) -> PG_DRV_W<16> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 20:21 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg5_drv(&mut self) -> PG_DRV_W<20> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 24:25 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg6_drv(&mut self) -> PG_DRV_W<24> {
        PG_DRV_W::new(self)
    }
    #[doc = "Bits 28:29 - PG Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg7_drv(&mut self) -> PG_DRV_W<28> {
        PG_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Multi_Driving Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_drv0](index.html) module"]
pub struct PG_DRV0_SPEC;
impl crate::RegisterSpec for PG_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_drv0::R](R) reader structure"]
impl crate::Readable for PG_DRV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_drv0::W](W) writer structure"]
impl crate::Writable for PG_DRV0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pg_drv0 to value 0"]
impl crate::Resettable for PG_DRV0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
