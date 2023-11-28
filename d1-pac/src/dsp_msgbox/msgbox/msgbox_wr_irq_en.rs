#[doc = "Register `msgbox_wr_irq_en` reader"]
pub type R = crate::R<MSGBOX_WR_IRQ_EN_SPEC>;
#[doc = "Register `msgbox_wr_irq_en` writer"]
pub type W = crate::W<MSGBOX_WR_IRQ_EN_SPEC>;
#[doc = "Field `transmit_mq_irq_en[0-3]` reader - Transmit Channel\\[i\\] Interrupt Enable"]
pub type TRANSMIT_MQ_IRQ_EN_R = crate::BitReader<TRANSMIT_MQ_IRQ_EN_A>;
#[doc = "Transmit Channel\\[i\\] Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANSMIT_MQ_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TRANSMIT_MQ_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TRANSMIT_MQ_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANSMIT_MQ_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRANSMIT_MQ_IRQ_EN_A {
        match self.bits {
            false => TRANSMIT_MQ_IRQ_EN_A::DISABLE,
            true => TRANSMIT_MQ_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRANSMIT_MQ_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TRANSMIT_MQ_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `transmit_mq_irq_en[0-3]` writer - Transmit Channel\\[i\\] Interrupt Enable"]
pub type TRANSMIT_MQ_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, TRANSMIT_MQ_IRQ_EN_A>;
impl<'a, REG> TRANSMIT_MQ_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TRANSMIT_MQ_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TRANSMIT_MQ_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Transmit Channel\\[i\\] Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `transmit_mq0_irq_en` field"]
    #[inline(always)]
    pub fn transmit_mq_irq_en(&self, n: u8) -> TRANSMIT_MQ_IRQ_EN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TRANSMIT_MQ_IRQ_EN_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn transmit_mq0_irq_en(&self) -> TRANSMIT_MQ_IRQ_EN_R {
        TRANSMIT_MQ_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn transmit_mq1_irq_en(&self) -> TRANSMIT_MQ_IRQ_EN_R {
        TRANSMIT_MQ_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn transmit_mq2_irq_en(&self) -> TRANSMIT_MQ_IRQ_EN_R {
        TRANSMIT_MQ_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn transmit_mq3_irq_en(&self) -> TRANSMIT_MQ_IRQ_EN_R {
        TRANSMIT_MQ_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Transmit Channel\\[i\\] Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `transmit_mq0_irq_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq_irq_en(&mut self, n: u8) -> TRANSMIT_MQ_IRQ_EN_W<MSGBOX_WR_IRQ_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TRANSMIT_MQ_IRQ_EN_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq0_irq_en(&mut self) -> TRANSMIT_MQ_IRQ_EN_W<MSGBOX_WR_IRQ_EN_SPEC> {
        TRANSMIT_MQ_IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq1_irq_en(&mut self) -> TRANSMIT_MQ_IRQ_EN_W<MSGBOX_WR_IRQ_EN_SPEC> {
        TRANSMIT_MQ_IRQ_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq2_irq_en(&mut self) -> TRANSMIT_MQ_IRQ_EN_W<MSGBOX_WR_IRQ_EN_SPEC> {
        TRANSMIT_MQ_IRQ_EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Transmit Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq3_irq_en(&mut self) -> TRANSMIT_MQ_IRQ_EN_W<MSGBOX_WR_IRQ_EN_SPEC> {
        TRANSMIT_MQ_IRQ_EN_W::new(self, 6)
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
#[doc = "Message Box Write Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_wr_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_wr_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_WR_IRQ_EN_SPEC;
impl crate::RegisterSpec for MSGBOX_WR_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_wr_irq_en::R`](R) reader structure"]
impl crate::Readable for MSGBOX_WR_IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_wr_irq_en::W`](W) writer structure"]
impl crate::Writable for MSGBOX_WR_IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_wr_irq_en to value 0"]
impl crate::Resettable for MSGBOX_WR_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
