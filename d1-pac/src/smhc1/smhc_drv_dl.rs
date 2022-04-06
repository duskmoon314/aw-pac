#[doc = "Register `SMHC_DRV_DL` reader"]
pub struct R(crate::R<SMHC_DRV_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_DRV_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_DRV_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_DRV_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_DRV_DL` writer"]
pub struct W(crate::W<SMHC_DRV_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_DRV_DL_SPEC>;
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
impl From<crate::W<SMHC_DRV_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_DRV_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_DRV_PH_SEL` reader - Data Drive Phase Select"]
pub struct DAT_DRV_PH_SEL_R(crate::FieldReader<bool, bool>);
impl DAT_DRV_PH_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAT_DRV_PH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_DRV_PH_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_DRV_PH_SEL` writer - Data Drive Phase Select"]
pub struct DAT_DRV_PH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_DRV_PH_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `CMD_DRV_PH_SEL` reader - Command Drive Phase Select"]
pub struct CMD_DRV_PH_SEL_R(crate::FieldReader<bool, bool>);
impl CMD_DRV_PH_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_DRV_PH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_DRV_PH_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_DRV_PH_SEL` writer - Command Drive Phase Select"]
pub struct CMD_DRV_PH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_DRV_PH_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Data Drive Phase Select"]
    #[inline(always)]
    pub fn dat_drv_ph_sel(&self) -> DAT_DRV_PH_SEL_R {
        DAT_DRV_PH_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Drive Phase Select"]
    #[inline(always)]
    pub fn cmd_drv_ph_sel(&self) -> CMD_DRV_PH_SEL_R {
        CMD_DRV_PH_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Data Drive Phase Select"]
    #[inline(always)]
    pub fn dat_drv_ph_sel(&mut self) -> DAT_DRV_PH_SEL_W {
        DAT_DRV_PH_SEL_W { w: self }
    }
    #[doc = "Bit 16 - Command Drive Phase Select"]
    #[inline(always)]
    pub fn cmd_drv_ph_sel(&mut self) -> CMD_DRV_PH_SEL_W {
        CMD_DRV_PH_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Drive Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_drv_dl](index.html) module"]
pub struct SMHC_DRV_DL_SPEC;
impl crate::RegisterSpec for SMHC_DRV_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_drv_dl::R](R) reader structure"]
impl crate::Readable for SMHC_DRV_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_drv_dl::W](W) writer structure"]
impl crate::Writable for SMHC_DRV_DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_DRV_DL to value 0"]
impl crate::Resettable for SMHC_DRV_DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
