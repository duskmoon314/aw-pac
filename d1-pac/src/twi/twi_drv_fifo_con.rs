#[doc = "Register `twi_drv_fifo_con` reader"]
pub type R = crate::R<TWI_DRV_FIFO_CON_SPEC>;
#[doc = "Register `twi_drv_fifo_con` writer"]
pub type W = crate::W<TWI_DRV_FIFO_CON_SPEC>;
#[doc = "Field `send_fifo_content` reader - "]
pub type SEND_FIFO_CONTENT_R = crate::FieldReader;
#[doc = "Field `send_fifo_content` writer - "]
pub type SEND_FIFO_CONTENT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `send_fifo_clear` reader - "]
pub type SEND_FIFO_CLEAR_R = crate::BitReader;
#[doc = "Field `send_fifo_clear` writer - "]
pub type SEND_FIFO_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `recv_fifo_content` reader - "]
pub type RECV_FIFO_CONTENT_R = crate::FieldReader;
#[doc = "Field `recv_fifo_content` writer - "]
pub type RECV_FIFO_CONTENT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `recv_fifo_clear` reader - "]
pub type RECV_FIFO_CLEAR_R = crate::BitReader;
#[doc = "Field `recv_fifo_clear` writer - "]
pub type RECV_FIFO_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn send_fifo_content(&self) -> SEND_FIFO_CONTENT_R {
        SEND_FIFO_CONTENT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn send_fifo_clear(&self) -> SEND_FIFO_CLEAR_R {
        SEND_FIFO_CLEAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn recv_fifo_content(&self) -> RECV_FIFO_CONTENT_R {
        RECV_FIFO_CONTENT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn recv_fifo_clear(&self) -> RECV_FIFO_CLEAR_R {
        RECV_FIFO_CLEAR_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn send_fifo_content(&mut self) -> SEND_FIFO_CONTENT_W<TWI_DRV_FIFO_CON_SPEC> {
        SEND_FIFO_CONTENT_W::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn send_fifo_clear(&mut self) -> SEND_FIFO_CLEAR_W<TWI_DRV_FIFO_CON_SPEC> {
        SEND_FIFO_CLEAR_W::new(self, 6)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn recv_fifo_content(&mut self) -> RECV_FIFO_CONTENT_W<TWI_DRV_FIFO_CON_SPEC> {
        RECV_FIFO_CONTENT_W::new(self, 16)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn recv_fifo_clear(&mut self) -> RECV_FIFO_CLEAR_W<TWI_DRV_FIFO_CON_SPEC> {
        RECV_FIFO_CLEAR_W::new(self, 22)
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
#[doc = "TWI_DRV FIFO Content Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_fifo_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_fifo_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_FIFO_CON_SPEC;
impl crate::RegisterSpec for TWI_DRV_FIFO_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_fifo_con::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_FIFO_CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_fifo_con::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_FIFO_CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_fifo_con to value 0"]
impl crate::Resettable for TWI_DRV_FIFO_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
