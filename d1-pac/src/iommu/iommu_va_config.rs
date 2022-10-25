#[doc = "Register `iommu_va_config` reader"]
pub struct R(crate::R<IOMMU_VA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_VA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_VA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_VA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_va_config` writer"]
pub struct W(crate::W<IOMMU_VA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_VA_CONFIG_SPEC>;
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
impl From<crate::W<IOMMU_VA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_VA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `va_config_start` reader - "]
pub type VA_CONFIG_START_R = crate::BitReader<VA_CONFIG_START_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VA_CONFIG_START_A {
    #[doc = "0: No operation or operation is completed"]
    NO_OPERATION_OR_COMPLETED = 0,
    #[doc = "1: Start"]
    S_TART = 1,
}
impl From<VA_CONFIG_START_A> for bool {
    #[inline(always)]
    fn from(variant: VA_CONFIG_START_A) -> Self {
        variant as u8 != 0
    }
}
impl VA_CONFIG_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VA_CONFIG_START_A {
        match self.bits {
            false => VA_CONFIG_START_A::NO_OPERATION_OR_COMPLETED,
            true => VA_CONFIG_START_A::S_TART,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION_OR_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_operation_or_completed(&self) -> bool {
        *self == VA_CONFIG_START_A::NO_OPERATION_OR_COMPLETED
    }
    #[doc = "Checks if the value of the field is `S_TART`"]
    #[inline(always)]
    pub fn is_s_tart(&self) -> bool {
        *self == VA_CONFIG_START_A::S_TART
    }
}
#[doc = "Field `va_config_start` writer - "]
pub type VA_CONFIG_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_VA_CONFIG_SPEC, VA_CONFIG_START_A, O>;
impl<'a, const O: u8> VA_CONFIG_START_W<'a, O> {
    #[doc = "No operation or operation is completed"]
    #[inline(always)]
    pub fn no_operation_or_completed(self) -> &'a mut W {
        self.variant(VA_CONFIG_START_A::NO_OPERATION_OR_COMPLETED)
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn s_tart(self) -> &'a mut W {
        self.variant(VA_CONFIG_START_A::S_TART)
    }
}
#[doc = "Field `va_config` reader - Virtual Address Configuration"]
pub type VA_CONFIG_R = crate::BitReader<VA_CONFIG_A>;
#[doc = "Virtual Address Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VA_CONFIG_A {
    #[doc = "0: Read operation"]
    R_EAD = 0,
    #[doc = "1: Write operation"]
    W_RITE = 1,
}
impl From<VA_CONFIG_A> for bool {
    #[inline(always)]
    fn from(variant: VA_CONFIG_A) -> Self {
        variant as u8 != 0
    }
}
impl VA_CONFIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VA_CONFIG_A {
        match self.bits {
            false => VA_CONFIG_A::R_EAD,
            true => VA_CONFIG_A::W_RITE,
        }
    }
    #[doc = "Checks if the value of the field is `R_EAD`"]
    #[inline(always)]
    pub fn is_r_ead(&self) -> bool {
        *self == VA_CONFIG_A::R_EAD
    }
    #[doc = "Checks if the value of the field is `W_RITE`"]
    #[inline(always)]
    pub fn is_w_rite(&self) -> bool {
        *self == VA_CONFIG_A::W_RITE
    }
}
#[doc = "Field `va_config` writer - Virtual Address Configuration"]
pub type VA_CONFIG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_VA_CONFIG_SPEC, VA_CONFIG_A, O>;
impl<'a, const O: u8> VA_CONFIG_W<'a, O> {
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn r_ead(self) -> &'a mut W {
        self.variant(VA_CONFIG_A::R_EAD)
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn w_rite(self) -> &'a mut W {
        self.variant(VA_CONFIG_A::W_RITE)
    }
}
#[doc = "Field `mode_sel` reader - "]
pub type MODE_SEL_R = crate::BitReader<MODE_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_SEL_A {
    #[doc = "0: Prefetch"]
    P_REFETCH = 0,
    #[doc = "1: Debug Mode"]
    D_EBUG = 1,
}
impl From<MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_SEL_A {
        match self.bits {
            false => MODE_SEL_A::P_REFETCH,
            true => MODE_SEL_A::D_EBUG,
        }
    }
    #[doc = "Checks if the value of the field is `P_REFETCH`"]
    #[inline(always)]
    pub fn is_p_refetch(&self) -> bool {
        *self == MODE_SEL_A::P_REFETCH
    }
    #[doc = "Checks if the value of the field is `D_EBUG`"]
    #[inline(always)]
    pub fn is_d_ebug(&self) -> bool {
        *self == MODE_SEL_A::D_EBUG
    }
}
#[doc = "Field `mode_sel` writer - "]
pub type MODE_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_VA_CONFIG_SPEC, MODE_SEL_A, O>;
impl<'a, const O: u8> MODE_SEL_W<'a, O> {
    #[doc = "Prefetch"]
    #[inline(always)]
    pub fn p_refetch(self) -> &'a mut W {
        self.variant(MODE_SEL_A::P_REFETCH)
    }
    #[doc = "Debug Mode"]
    #[inline(always)]
    pub fn d_ebug(self) -> &'a mut W {
        self.variant(MODE_SEL_A::D_EBUG)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn va_config_start(&self) -> VA_CONFIG_START_R {
        VA_CONFIG_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Virtual Address Configuration"]
    #[inline(always)]
    pub fn va_config(&self) -> VA_CONFIG_R {
        VA_CONFIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mode_sel(&self) -> MODE_SEL_R {
        MODE_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn va_config_start(&mut self) -> VA_CONFIG_START_W<0> {
        VA_CONFIG_START_W::new(self)
    }
    #[doc = "Bit 8 - Virtual Address Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn va_config(&mut self) -> VA_CONFIG_W<8> {
        VA_CONFIG_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn mode_sel(&mut self) -> MODE_SEL_W<31> {
        MODE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Virtual Address Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_va_config](index.html) module"]
pub struct IOMMU_VA_CONFIG_SPEC;
impl crate::RegisterSpec for IOMMU_VA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_va_config::R](R) reader structure"]
impl crate::Readable for IOMMU_VA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_va_config::W](W) writer structure"]
impl crate::Writable for IOMMU_VA_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_va_config to value 0"]
impl crate::Resettable for IOMMU_VA_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
