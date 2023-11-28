#[doc = "Register `twi_drv_fmt` reader"]
pub type R = crate::R<TWI_DRV_FMT_SPEC>;
#[doc = "Register `twi_drv_fmt` writer"]
pub type W = crate::W<TWI_DRV_FMT_SPEC>;
#[doc = "Field `data_byte` reader - "]
pub type DATA_BYTE_R = crate::FieldReader<u16>;
#[doc = "Field `data_byte` writer - "]
pub type DATA_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `addr_byte` reader - "]
pub type ADDR_BYTE_R = crate::FieldReader;
#[doc = "Field `addr_byte` writer - "]
pub type ADDR_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr_byte(&self) -> ADDR_BYTE_R {
        ADDR_BYTE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data_byte(&mut self) -> DATA_BYTE_W<TWI_DRV_FMT_SPEC> {
        DATA_BYTE_W::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn addr_byte(&mut self) -> ADDR_BYTE_W<TWI_DRV_FMT_SPEC> {
        ADDR_BYTE_W::new(self, 16)
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
#[doc = "TWI_DRV Packet Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_fmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_fmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_FMT_SPEC;
impl crate::RegisterSpec for TWI_DRV_FMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_fmt::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_FMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_fmt::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_FMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_fmt to value 0"]
impl crate::Resettable for TWI_DRV_FMT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
