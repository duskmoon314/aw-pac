#[doc = "Register `emmc_ddr_sbit_det` reader"]
pub type R = crate::R<EMMC_DDR_SBIT_DET_SPEC>;
#[doc = "Register `emmc_ddr_sbit_det` writer"]
pub type W = crate::W<EMMC_DDR_SBIT_DET_SPEC>;
#[doc = "Field `half_start_bit` reader - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
pub type HALF_START_BIT_R = crate::BitReader<HALF_START_BIT_A>;
#[doc = "Control for start bit detection mechanism inside mstorage based on duration of start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALF_START_BIT_A {
    #[doc = "0: Full cycle"]
    FULL = 0,
    #[doc = "1: Less than one full cycle"]
    LESS = 1,
}
impl From<HALF_START_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_START_BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALF_START_BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HALF_START_BIT_A {
        match self.bits {
            false => HALF_START_BIT_A::FULL,
            true => HALF_START_BIT_A::LESS,
        }
    }
    #[doc = "Full cycle"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == HALF_START_BIT_A::FULL
    }
    #[doc = "Less than one full cycle"]
    #[inline(always)]
    pub fn is_less(&self) -> bool {
        *self == HALF_START_BIT_A::LESS
    }
}
#[doc = "Field `half_start_bit` writer - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
pub type HALF_START_BIT_W<'a, REG> = crate::BitWriter<'a, REG, HALF_START_BIT_A>;
impl<'a, REG> HALF_START_BIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full cycle"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(HALF_START_BIT_A::FULL)
    }
    #[doc = "Less than one full cycle"]
    #[inline(always)]
    pub fn less(self) -> &'a mut crate::W<REG> {
        self.variant(HALF_START_BIT_A::LESS)
    }
}
#[doc = "Field `hs400_md_en` reader - HS400 Mode Enable"]
pub type HS400_MD_EN_R = crate::BitReader<HS400_MD_EN_A>;
#[doc = "HS400 Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS400_MD_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HS400_MD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HS400_MD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HS400_MD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HS400_MD_EN_A {
        match self.bits {
            false => HS400_MD_EN_A::DISABLED,
            true => HS400_MD_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HS400_MD_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HS400_MD_EN_A::ENABLED
    }
}
#[doc = "Field `hs400_md_en` writer - HS400 Mode Enable"]
pub type HS400_MD_EN_W<'a, REG> = crate::BitWriter<'a, REG, HS400_MD_EN_A>;
impl<'a, REG> HS400_MD_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HS400_MD_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HS400_MD_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
    #[inline(always)]
    pub fn half_start_bit(&self) -> HALF_START_BIT_R {
        HALF_START_BIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - HS400 Mode Enable"]
    #[inline(always)]
    pub fn hs400_md_en(&self) -> HS400_MD_EN_R {
        HS400_MD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
    #[inline(always)]
    #[must_use]
    pub fn half_start_bit(&mut self) -> HALF_START_BIT_W<EMMC_DDR_SBIT_DET_SPEC> {
        HALF_START_BIT_W::new(self, 0)
    }
    #[doc = "Bit 31 - HS400 Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_md_en(&mut self) -> HS400_MD_EN_W<EMMC_DDR_SBIT_DET_SPEC> {
        HS400_MD_EN_W::new(self, 31)
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
#[doc = "eMMC4.5 DDR Start Bit Detection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmc_ddr_sbit_det::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmc_ddr_sbit_det::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMMC_DDR_SBIT_DET_SPEC;
impl crate::RegisterSpec for EMMC_DDR_SBIT_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmc_ddr_sbit_det::R`](R) reader structure"]
impl crate::Readable for EMMC_DDR_SBIT_DET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emmc_ddr_sbit_det::W`](W) writer structure"]
impl crate::Writable for EMMC_DDR_SBIT_DET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emmc_ddr_sbit_det to value 0"]
impl crate::Resettable for EMMC_DDR_SBIT_DET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
