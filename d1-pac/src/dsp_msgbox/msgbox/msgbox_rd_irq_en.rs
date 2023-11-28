#[doc = "Register `msgbox_rd_irq_en` reader"]
pub type R = crate::R<MSGBOX_RD_IRQ_EN_SPEC>;
#[doc = "Register `msgbox_rd_irq_en` writer"]
pub type W = crate::W<MSGBOX_RD_IRQ_EN_SPEC>;
#[doc = "Field `reception_mq_irq_en[0-3]` reader - Reception Channel\\[i\\] Interrupt Enable"]
pub type RECEPTION_MQ_IRQ_EN_R = crate::BitReader<RECEPTION_MQ_IRQ_EN_A>;
#[doc = "Reception Channel\\[i\\] Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECEPTION_MQ_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RECEPTION_MQ_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RECEPTION_MQ_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEPTION_MQ_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RECEPTION_MQ_IRQ_EN_A {
        match self.bits {
            false => RECEPTION_MQ_IRQ_EN_A::DISABLE,
            true => RECEPTION_MQ_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RECEPTION_MQ_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RECEPTION_MQ_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `reception_mq_irq_en[0-3]` writer - Reception Channel\\[i\\] Interrupt Enable"]
pub type RECEPTION_MQ_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, RECEPTION_MQ_IRQ_EN_A>;
impl<'a, REG> RECEPTION_MQ_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RECEPTION_MQ_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RECEPTION_MQ_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Reception Channel\\[i\\] Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `reception_mq0_irq_en` field"]
    #[inline(always)]
    pub fn reception_mq_irq_en(&self, n: u8) -> RECEPTION_MQ_IRQ_EN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq0_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq1_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq2_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq3_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Reception Channel\\[i\\] Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `reception_mq0_irq_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq_irq_en(&mut self, n: u8) -> RECEPTION_MQ_IRQ_EN_W<MSGBOX_RD_IRQ_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RECEPTION_MQ_IRQ_EN_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq0_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<MSGBOX_RD_IRQ_EN_SPEC> {
        RECEPTION_MQ_IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq1_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<MSGBOX_RD_IRQ_EN_SPEC> {
        RECEPTION_MQ_IRQ_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq2_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<MSGBOX_RD_IRQ_EN_SPEC> {
        RECEPTION_MQ_IRQ_EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq3_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<MSGBOX_RD_IRQ_EN_SPEC> {
        RECEPTION_MQ_IRQ_EN_W::new(self, 6)
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
#[doc = "Message Box Read Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_rd_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_rd_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_RD_IRQ_EN_SPEC;
impl crate::RegisterSpec for MSGBOX_RD_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_rd_irq_en::R`](R) reader structure"]
impl crate::Readable for MSGBOX_RD_IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_rd_irq_en::W`](W) writer structure"]
impl crate::Writable for MSGBOX_RD_IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_rd_irq_en to value 0"]
impl crate::Resettable for MSGBOX_RD_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
