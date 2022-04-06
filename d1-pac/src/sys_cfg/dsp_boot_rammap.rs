#[doc = "Register `DSP_BOOT_RAMMAP` reader"]
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
#[doc = "Register `DSP_BOOT_RAMMAP` writer"]
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DSP_BOOT_SRAM_REMAP_ENABLE` reader - "]
pub struct DSP_BOOT_SRAM_REMAP_ENABLE_R(crate::FieldReader<bool, DSP_BOOT_SRAM_REMAP_ENABLE_A>);
impl DSP_BOOT_SRAM_REMAP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSP_BOOT_SRAM_REMAP_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DSP_BOOT_SRAM_REMAP_ENABLE_A::DSP_SYS
    }
    #[doc = "Checks if the value of the field is `SYS_BOOT`"]
    #[inline(always)]
    pub fn is_sys_boot(&self) -> bool {
        **self == DSP_BOOT_SRAM_REMAP_ENABLE_A::SYS_BOOT
    }
}
impl core::ops::Deref for DSP_BOOT_SRAM_REMAP_ENABLE_R {
    type Target = crate::FieldReader<bool, DSP_BOOT_SRAM_REMAP_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSP_BOOT_SRAM_REMAP_ENABLE` writer - "]
pub struct DSP_BOOT_SRAM_REMAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_BOOT_SRAM_REMAP_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_BOOT_SRAM_REMAP_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
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
    pub fn dsp_boot_sram_remap_enable(&mut self) -> DSP_BOOT_SRAM_REMAP_ENABLE_W {
        DSP_BOOT_SRAM_REMAP_ENABLE_W { w: self }
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
}
#[doc = "`reset()` method sets DSP_BOOT_RAMMAP to value 0"]
impl crate::Resettable for DSP_BOOT_RAMMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
