#[doc = "Register `hsk` reader"]
pub type R = crate::R<HSK_SPEC>;
#[doc = "Register `hsk` writer"]
pub type W = crate::W<HSK_SPEC>;
#[doc = "Field `hsk` reader - Handshake configuration"]
pub type HSK_R = crate::FieldReader<HSK_A>;
#[doc = "Handshake configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSK_A {
    #[doc = "165: `10100101`"]
    WAIT_CYCLE = 165,
    #[doc = "229: `11100101`"]
    HANDSHAKE = 229,
}
impl From<HSK_A> for u8 {
    #[inline(always)]
    fn from(variant: HSK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSK_A {
    type Ux = u8;
}
impl HSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HSK_A> {
        match self.bits {
            165 => Some(HSK_A::WAIT_CYCLE),
            229 => Some(HSK_A::HANDSHAKE),
            _ => None,
        }
    }
    #[doc = "`10100101`"]
    #[inline(always)]
    pub fn is_wait_cycle(&self) -> bool {
        *self == HSK_A::WAIT_CYCLE
    }
    #[doc = "`11100101`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == HSK_A::HANDSHAKE
    }
}
#[doc = "Field `hsk` writer - Handshake configuration"]
pub type HSK_W<'a, REG> = crate::FieldWriter<'a, REG, 8, HSK_A>;
impl<'a, REG> HSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10100101`"]
    #[inline(always)]
    pub fn wait_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(HSK_A::WAIT_CYCLE)
    }
    #[doc = "`11100101`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut crate::W<REG> {
        self.variant(HSK_A::HANDSHAKE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Handshake configuration"]
    #[inline(always)]
    pub fn hsk(&self) -> HSK_R {
        HSK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Handshake configuration"]
    #[inline(always)]
    #[must_use]
    pub fn hsk(&mut self) -> HSK_W<HSK_SPEC> {
        HSK_W::new(self, 0)
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
#[doc = "UART DMA Handshake Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSK_SPEC;
impl crate::RegisterSpec for HSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsk::R`](R) reader structure"]
impl crate::Readable for HSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsk::W`](W) writer structure"]
impl crate::Writable for HSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hsk to value 0"]
impl crate::Resettable for HSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
