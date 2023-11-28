#[doc = "Register `lvds_bgr` reader"]
pub type R = crate::R<LVDS_BGR_SPEC>;
#[doc = "Register `lvds_bgr` writer"]
pub type W = crate::W<LVDS_BGR_SPEC>;
#[doc = "Field `rst` reader - Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::ASSERT,
            true => RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == RST_A::DEASSERT
    }
}
#[doc = "Field `rst` writer - Reset"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG, RST_A>;
impl<'a, REG> RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<LVDS_BGR_SPEC> {
        RST_W::new(self, 16)
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
#[doc = "LVDS Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvds_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvds_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVDS_BGR_SPEC;
impl crate::RegisterSpec for LVDS_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvds_bgr::R`](R) reader structure"]
impl crate::Readable for LVDS_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvds_bgr::W`](W) writer structure"]
impl crate::Writable for LVDS_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lvds_bgr to value 0"]
impl crate::Resettable for LVDS_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
