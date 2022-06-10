#[doc = "Register `TWI_DRV_FMT` reader"]
pub struct R(crate::R<TWI_DRV_FMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_FMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_FMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_FMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_DRV_FMT` writer"]
pub struct W(crate::W<TWI_DRV_FMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_FMT_SPEC>;
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
impl From<crate::W<TWI_DRV_FMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_FMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `addr_byte` reader - "]
pub type ADDR_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `addr_byte` writer - "]
pub type ADDR_BYTE_W<'a> = crate::FieldWriter<'a, u32, TWI_DRV_FMT_SPEC, u8, u8, 8, 16>;
#[doc = "Field `data_byte` reader - "]
pub type DATA_BYTE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `data_byte` writer - "]
pub type DATA_BYTE_W<'a> = crate::FieldWriter<'a, u32, TWI_DRV_FMT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr_byte(&self) -> ADDR_BYTE_R {
        ADDR_BYTE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr_byte(&mut self) -> ADDR_BYTE_W {
        ADDR_BYTE_W::new(self)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data_byte(&mut self) -> DATA_BYTE_W {
        DATA_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Packet Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_fmt](index.html) module"]
pub struct TWI_DRV_FMT_SPEC;
impl crate::RegisterSpec for TWI_DRV_FMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_fmt::R](R) reader structure"]
impl crate::Readable for TWI_DRV_FMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_fmt::W](W) writer structure"]
impl crate::Writable for TWI_DRV_FMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_DRV_FMT to value 0"]
impl crate::Resettable for TWI_DRV_FMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
