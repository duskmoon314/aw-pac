#[doc = "Register `cir_rxpcfg` reader"]
pub type R = crate::R<CIR_RXPCFG_SPEC>;
#[doc = "Register `cir_rxpcfg` writer"]
pub type W = crate::W<CIR_RXPCFG_SPEC>;
#[doc = "Field `rppi` reader - Receiver Pulse Polarity Invert"]
pub type RPPI_R = crate::BitReader<RPPI_A>;
#[doc = "Receiver Pulse Polarity Invert\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPPI_A {
    #[doc = "0: Do not invert receiver signal"]
    NOT_INVERT = 0,
    #[doc = "1: Invert receiver signal"]
    INVERT = 1,
}
impl From<RPPI_A> for bool {
    #[inline(always)]
    fn from(variant: RPPI_A) -> Self {
        variant as u8 != 0
    }
}
impl RPPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPPI_A {
        match self.bits {
            false => RPPI_A::NOT_INVERT,
            true => RPPI_A::INVERT,
        }
    }
    #[doc = "Do not invert receiver signal"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == RPPI_A::NOT_INVERT
    }
    #[doc = "Invert receiver signal"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == RPPI_A::INVERT
    }
}
#[doc = "Field `rppi` writer - Receiver Pulse Polarity Invert"]
pub type RPPI_W<'a, REG> = crate::BitWriter<'a, REG, RPPI_A>;
impl<'a, REG> RPPI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not invert receiver signal"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut crate::W<REG> {
        self.variant(RPPI_A::NOT_INVERT)
    }
    #[doc = "Invert receiver signal"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(RPPI_A::INVERT)
    }
}
impl R {
    #[doc = "Bit 2 - Receiver Pulse Polarity Invert"]
    #[inline(always)]
    pub fn rppi(&self) -> RPPI_R {
        RPPI_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver Pulse Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rppi(&mut self) -> RPPI_W<CIR_RXPCFG_SPEC> {
        RPPI_W::new(self, 2)
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
#[doc = "CIR Receiver Pulse Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_RXPCFG_SPEC;
impl crate::RegisterSpec for CIR_RXPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_rxpcfg::R`](R) reader structure"]
impl crate::Readable for CIR_RXPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_rxpcfg::W`](W) writer structure"]
impl crate::Writable for CIR_RXPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_rxpcfg to value 0x04"]
impl crate::Resettable for CIR_RXPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
