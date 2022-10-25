#[doc = "Register `dsp_boot_rammap` reader"]
pub struct R(crate::R<DSP_BOOT_RAMMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_BOOT_RAMMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_BOOT_RAMMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_BOOT_RAMMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dsp_boot_rammap` writer"]
pub struct W(crate::W<DSP_BOOT_RAMMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_BOOT_RAMMAP_SPEC>;
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
impl From<crate::W<DSP_BOOT_RAMMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_BOOT_RAMMAP_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> DSP_BOOT_SRAM_REMAP_ENABLE_A {
        match self.bits {
            false => DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS,
            true => DSP_BOOT_SRAM_REMAP_ENABLE_A::SYS_BOOT,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_SYS`"]
    #[inline(always)]
    pub fn is_dsp_sys(&self) -> bool {
        *self == DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS
    }
    #[doc = "Checks if the value of the field is `SYS_BOOT`"]
    #[inline(always)]
    pub fn is_sys_boot(&self) -> bool {
        *self == DSP_BOOT_SRAM_REMAP_ENABLE_A::SYS_BOOT
    }
}
#[doc = "Field `dsp_boot_sram_remap_enable` writer - "]
pub type DSP_BOOT_SRAM_REMAP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_BOOT_RAMMAP_SPEC, DSP_BOOT_SRAM_REMAP_ENABLE_A, O>;
impl<'a, const O: u8> DSP_BOOT_SRAM_REMAP_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dsp_sys(self) -> &'a mut W {
        self.variant(DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sys_boot(self) -> &'a mut W {
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
    pub fn dsp_boot_sram_remap_enable(&mut self) -> DSP_BOOT_SRAM_REMAP_ENABLE_W<0> {
        DSP_BOOT_SRAM_REMAP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP Boot SRAM Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_boot_rammap](index.html) module"]
pub struct DSP_BOOT_RAMMAP_SPEC;
impl crate::RegisterSpec for DSP_BOOT_RAMMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_boot_rammap::R](R) reader structure"]
impl crate::Readable for DSP_BOOT_RAMMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_boot_rammap::W](W) writer structure"]
impl crate::Writable for DSP_BOOT_RAMMAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dsp_boot_rammap to value 0"]
impl crate::Resettable for DSP_BOOT_RAMMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
