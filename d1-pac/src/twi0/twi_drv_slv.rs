#[doc = "Register `TWI_DRV_SLV` reader"]
pub struct R(crate::R<TWI_DRV_SLV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_SLV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_SLV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_SLV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_DRV_SLV` writer"]
pub struct W(crate::W<TWI_DRV_SLV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_SLV_SPEC>;
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
impl From<crate::W<TWI_DRV_SLV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_SLV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slv_id` reader - Slave device ID"]
pub struct SLV_ID_R(crate::FieldReader<u8, u8>);
impl SLV_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slv_id` writer - Slave device ID"]
pub struct SLV_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "R/W operation to slave device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_A {
    #[doc = "0: `0`"]
    WRITE = 0,
    #[doc = "1: `1`"]
    READ = 1,
}
impl From<CMD_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmd` reader - R/W operation to slave device"]
pub struct CMD_R(crate::FieldReader<bool, CMD_A>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_A {
        match self.bits {
            false => CMD_A::WRITE,
            true => CMD_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == CMD_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == CMD_A::READ
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<bool, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmd` writer - R/W operation to slave device"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_A::WRITE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_A::READ)
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `slv_id_x` reader - SLAX\\[7:0\\]"]
pub struct SLV_ID_X_R(crate::FieldReader<u8, u8>);
impl SLV_ID_X_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_ID_X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ID_X_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slv_id_x` writer - SLAX\\[7:0\\]"]
pub struct SLV_ID_X_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_ID_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:15 - Slave device ID"]
    #[inline(always)]
    pub fn slv_id(&self) -> SLV_ID_R {
        SLV_ID_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - R/W operation to slave device"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - SLAX\\[7:0\\]"]
    #[inline(always)]
    pub fn slv_id_x(&self) -> SLV_ID_X_R {
        SLV_ID_X_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 9:15 - Slave device ID"]
    #[inline(always)]
    pub fn slv_id(&mut self) -> SLV_ID_W {
        SLV_ID_W { w: self }
    }
    #[doc = "Bit 8 - R/W operation to slave device"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 0:7 - SLAX\\[7:0\\]"]
    #[inline(always)]
    pub fn slv_id_x(&mut self) -> SLV_ID_X_W {
        SLV_ID_X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Slave ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_slv](index.html) module"]
pub struct TWI_DRV_SLV_SPEC;
impl crate::RegisterSpec for TWI_DRV_SLV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_slv::R](R) reader structure"]
impl crate::Readable for TWI_DRV_SLV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_slv::W](W) writer structure"]
impl crate::Writable for TWI_DRV_SLV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_DRV_SLV to value 0"]
impl crate::Resettable for TWI_DRV_SLV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
