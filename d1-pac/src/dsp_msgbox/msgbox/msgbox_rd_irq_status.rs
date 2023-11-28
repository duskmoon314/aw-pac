#[doc = "Register `msgbox_rd_irq_status` reader"]
pub type R = crate::R<MSGBOX_RD_IRQ_STATUS_SPEC>;
#[doc = "Register `msgbox_rd_irq_status` writer"]
pub type W = crate::W<MSGBOX_RD_IRQ_STATUS_SPEC>;
#[doc = "Field `reception_mq_irq_pend[0-3]` reader - Reception Channel\\[i\\] Interrupt Pending"]
pub type RECEPTION_MQ_IRQ_PEND_R = crate::BitReader<RECEPTION_MQ_IRQ_PEND_A>;
#[doc = "Reception Channel\\[i\\] Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECEPTION_MQ_IRQ_PEND_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Pending"]
    PENDING = 1,
}
impl From<RECEPTION_MQ_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: RECEPTION_MQ_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEPTION_MQ_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RECEPTION_MQ_IRQ_PEND_A {
        match self.bits {
            false => RECEPTION_MQ_IRQ_PEND_A::NO_EFFECT,
            true => RECEPTION_MQ_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RECEPTION_MQ_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEPTION_MQ_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `reception_mq_irq_pend[0-3]` writer - Reception Channel\\[i\\] Interrupt Pending"]
pub type RECEPTION_MQ_IRQ_PEND_W<'a, REG> = crate::BitWriter1C<'a, REG, RECEPTION_MQ_IRQ_PEND_A>;
impl<'a, REG> RECEPTION_MQ_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RECEPTION_MQ_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RECEPTION_MQ_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "Reception Channel\\[i\\] Interrupt Pending\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `reception_mq0_irq_pend` field"]
    #[inline(always)]
    pub fn reception_mq_irq_pend(&self, n: u8) -> RECEPTION_MQ_IRQ_PEND_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RECEPTION_MQ_IRQ_PEND_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn reception_mq0_irq_pend(&self) -> RECEPTION_MQ_IRQ_PEND_R {
        RECEPTION_MQ_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn reception_mq1_irq_pend(&self) -> RECEPTION_MQ_IRQ_PEND_R {
        RECEPTION_MQ_IRQ_PEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn reception_mq2_irq_pend(&self) -> RECEPTION_MQ_IRQ_PEND_R {
        RECEPTION_MQ_IRQ_PEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn reception_mq3_irq_pend(&self) -> RECEPTION_MQ_IRQ_PEND_R {
        RECEPTION_MQ_IRQ_PEND_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Reception Channel\\[i\\] Interrupt Pending\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `reception_mq0_irq_pend` field"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq_irq_pend(
        &mut self,
        n: u8,
    ) -> RECEPTION_MQ_IRQ_PEND_W<MSGBOX_RD_IRQ_STATUS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RECEPTION_MQ_IRQ_PEND_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq0_irq_pend(&mut self) -> RECEPTION_MQ_IRQ_PEND_W<MSGBOX_RD_IRQ_STATUS_SPEC> {
        RECEPTION_MQ_IRQ_PEND_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq1_irq_pend(&mut self) -> RECEPTION_MQ_IRQ_PEND_W<MSGBOX_RD_IRQ_STATUS_SPEC> {
        RECEPTION_MQ_IRQ_PEND_W::new(self, 2)
    }
    #[doc = "Bit 4 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq2_irq_pend(&mut self) -> RECEPTION_MQ_IRQ_PEND_W<MSGBOX_RD_IRQ_STATUS_SPEC> {
        RECEPTION_MQ_IRQ_PEND_W::new(self, 4)
    }
    #[doc = "Bit 6 - Reception Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq3_irq_pend(&mut self) -> RECEPTION_MQ_IRQ_PEND_W<MSGBOX_RD_IRQ_STATUS_SPEC> {
        RECEPTION_MQ_IRQ_PEND_W::new(self, 6)
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
#[doc = "Message Box Read Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_rd_irq_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_rd_irq_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_RD_IRQ_STATUS_SPEC;
impl crate::RegisterSpec for MSGBOX_RD_IRQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_rd_irq_status::R`](R) reader structure"]
impl crate::Readable for MSGBOX_RD_IRQ_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_rd_irq_status::W`](W) writer structure"]
impl crate::Writable for MSGBOX_RD_IRQ_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets msgbox_rd_irq_status to value 0"]
impl crate::Resettable for MSGBOX_RD_IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
