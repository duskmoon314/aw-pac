#[doc = "Register `dsp_boot_rammap` reader"]
pub type R = crate::R<DSP_BOOT_RAMMAP_SPEC>;
#[doc = "Register `dsp_boot_rammap` writer"]
pub type W = crate::W<DSP_BOOT_RAMMAP_SPEC>;
#[doc = "Field `dsp_boot_sram_remap_enable` reader - "]
pub type DSP_BOOT_SRAM_REMAP_ENABLE_R = crate::BitReader<DSP_BOOT_SRAM_REMAP_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_BOOT_SRAM_REMAP_ENABLE_A {
    #[doc = "0: `0`"]
    DSP_SYS = 0,
    #[doc = "1: `1`"]
    SYS_BOOT = 1,
}
impl From<DSP_BOOT_SRAM_REMAP_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_BOOT_SRAM_REMAP_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DSP_BOOT_SRAM_REMAP_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSP_BOOT_SRAM_REMAP_ENABLE_A {
        match self.bits {
            false => DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS,
            true => DSP_BOOT_SRAM_REMAP_ENABLE_A::SYS_BOOT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_dsp_sys(&self) -> bool {
        *self == DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sys_boot(&self) -> bool {
        *self == DSP_BOOT_SRAM_REMAP_ENABLE_A::SYS_BOOT
    }
}
#[doc = "Field `dsp_boot_sram_remap_enable` writer - "]
pub type DSP_BOOT_SRAM_REMAP_ENABLE_W<'a, REG> =
    crate::BitWriter<'a, REG, DSP_BOOT_SRAM_REMAP_ENABLE_A>;
impl<'a, REG> DSP_BOOT_SRAM_REMAP_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dsp_sys(self) -> &'a mut crate::W<REG> {
        self.variant(DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sys_boot(self) -> &'a mut crate::W<REG> {
        self.variant(DSP_BOOT_SRAM_REMAP_ENABLE_A::SYS_BOOT)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dsp_boot_sram_remap_enable(&self) -> DSP_BOOT_SRAM_REMAP_ENABLE_R {
        DSP_BOOT_SRAM_REMAP_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_boot_sram_remap_enable(
        &mut self,
    ) -> DSP_BOOT_SRAM_REMAP_ENABLE_W<DSP_BOOT_RAMMAP_SPEC> {
        DSP_BOOT_SRAM_REMAP_ENABLE_W::new(self, 0)
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
#[doc = "DSP Boot SRAM Remap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_boot_rammap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_boot_rammap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSP_BOOT_RAMMAP_SPEC;
impl crate::RegisterSpec for DSP_BOOT_RAMMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_boot_rammap::R`](R) reader structure"]
impl crate::Readable for DSP_BOOT_RAMMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsp_boot_rammap::W`](W) writer structure"]
impl crate::Writable for DSP_BOOT_RAMMAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dsp_boot_rammap to value 0"]
impl crate::Resettable for DSP_BOOT_RAMMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
