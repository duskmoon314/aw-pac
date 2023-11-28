#[doc = "Register `iommu_va_config` reader"]
pub type R = crate::R<IOMMU_VA_CONFIG_SPEC>;
#[doc = "Register `iommu_va_config` writer"]
pub type W = crate::W<IOMMU_VA_CONFIG_SPEC>;
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
    pub const fn variant(&self) -> VA_CONFIG_START_A {
        match self.bits {
            false => VA_CONFIG_START_A::NO_OPERATION_OR_COMPLETED,
            true => VA_CONFIG_START_A::S_TART,
        }
    }
    #[doc = "No operation or operation is completed"]
    #[inline(always)]
    pub fn is_no_operation_or_completed(&self) -> bool {
        *self == VA_CONFIG_START_A::NO_OPERATION_OR_COMPLETED
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn is_s_tart(&self) -> bool {
        *self == VA_CONFIG_START_A::S_TART
    }
}
#[doc = "Field `va_config_start` writer - "]
pub type VA_CONFIG_START_W<'a, REG> = crate::BitWriter<'a, REG, VA_CONFIG_START_A>;
impl<'a, REG> VA_CONFIG_START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation or operation is completed"]
    #[inline(always)]
    pub fn no_operation_or_completed(self) -> &'a mut crate::W<REG> {
        self.variant(VA_CONFIG_START_A::NO_OPERATION_OR_COMPLETED)
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn s_tart(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> VA_CONFIG_A {
        match self.bits {
            false => VA_CONFIG_A::R_EAD,
            true => VA_CONFIG_A::W_RITE,
        }
    }
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn is_r_ead(&self) -> bool {
        *self == VA_CONFIG_A::R_EAD
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn is_w_rite(&self) -> bool {
        *self == VA_CONFIG_A::W_RITE
    }
}
#[doc = "Field `va_config` writer - Virtual Address Configuration"]
pub type VA_CONFIG_W<'a, REG> = crate::BitWriter<'a, REG, VA_CONFIG_A>;
impl<'a, REG> VA_CONFIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn r_ead(self) -> &'a mut crate::W<REG> {
        self.variant(VA_CONFIG_A::R_EAD)
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn w_rite(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> MODE_SEL_A {
        match self.bits {
            false => MODE_SEL_A::P_REFETCH,
            true => MODE_SEL_A::D_EBUG,
        }
    }
    #[doc = "Prefetch"]
    #[inline(always)]
    pub fn is_p_refetch(&self) -> bool {
        *self == MODE_SEL_A::P_REFETCH
    }
    #[doc = "Debug Mode"]
    #[inline(always)]
    pub fn is_d_ebug(&self) -> bool {
        *self == MODE_SEL_A::D_EBUG
    }
}
#[doc = "Field `mode_sel` writer - "]
pub type MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG, MODE_SEL_A>;
impl<'a, REG> MODE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch"]
    #[inline(always)]
    pub fn p_refetch(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_SEL_A::P_REFETCH)
    }
    #[doc = "Debug Mode"]
    #[inline(always)]
    pub fn d_ebug(self) -> &'a mut crate::W<REG> {
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
    pub fn va_config_start(&mut self) -> VA_CONFIG_START_W<IOMMU_VA_CONFIG_SPEC> {
        VA_CONFIG_START_W::new(self, 0)
    }
    #[doc = "Bit 8 - Virtual Address Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn va_config(&mut self) -> VA_CONFIG_W<IOMMU_VA_CONFIG_SPEC> {
        VA_CONFIG_W::new(self, 8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn mode_sel(&mut self) -> MODE_SEL_W<IOMMU_VA_CONFIG_SPEC> {
        MODE_SEL_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IOMMU Virtual Address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_va_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_va_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_VA_CONFIG_SPEC;
impl crate::RegisterSpec for IOMMU_VA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_va_config::R`](R) reader structure"]
impl crate::Readable for IOMMU_VA_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_va_config::W`](W) writer structure"]
impl crate::Writable for IOMMU_VA_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_va_config to value 0"]
impl crate::Resettable for IOMMU_VA_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
