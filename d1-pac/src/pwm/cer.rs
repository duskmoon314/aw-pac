#[doc = "Register `cer` reader"]
pub type R = crate::R<CER_SPEC>;
#[doc = "Register `cer` writer"]
pub type W = crate::W<CER_SPEC>;
#[doc = "Field `cap_en[0-7]` reader - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
pub type CAP_EN_R = crate::BitReader<CAP_EN_A>;
#[doc = "When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP_EN_A {
    #[doc = "0: Capture disable"]
    DISABLE = 0,
    #[doc = "1: Capture enable"]
    ENABLE = 1,
}
impl From<CAP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CAP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP_EN_A {
        match self.bits {
            false => CAP_EN_A::DISABLE,
            true => CAP_EN_A::ENABLE,
        }
    }
    #[doc = "Capture disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP_EN_A::DISABLE
    }
    #[doc = "Capture enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP_EN_A::ENABLE
    }
}
#[doc = "Field `cap_en[0-7]` writer - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
pub type CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG, CAP_EN_A>;
impl<'a, REG> CAP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP_EN_A::DISABLE)
    }
    #[doc = "Capture enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cap0_en` field"]
    #[inline(always)]
    pub fn cap_en(&self, n: u8) -> CAP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CAP_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP_EN_R {
        CAP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap3_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap4_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap5_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap6_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap7_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cap0_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn cap_en(&mut self, n: u8) -> CAP_EN_W<CER_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CAP_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap0_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap2_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap3_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap4_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap5_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap6_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap7_en(&mut self) -> CAP_EN_W<CER_SPEC> {
        CAP_EN_W::new(self, 7)
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
#[doc = "Capture Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CER_SPEC;
impl crate::RegisterSpec for CER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cer::R`](R) reader structure"]
impl crate::Readable for CER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cer::W`](W) writer structure"]
impl crate::Writable for CER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cer to value 0"]
impl crate::Resettable for CER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
