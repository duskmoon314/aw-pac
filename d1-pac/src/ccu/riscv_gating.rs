#[doc = "Register `riscv_gating` reader"]
pub type R = crate::R<RISCV_GATING_SPEC>;
#[doc = "Register `riscv_gating` writer"]
pub type W = crate::W<RISCV_GATING_SPEC>;
#[doc = "Field `gating_field` reader - "]
pub type GATING_FIELD_R = crate::FieldReader<u16>;
#[doc = "Field `gating_field` writer - "]
pub type GATING_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `gating` reader - Gating Clock"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::MASK,
            true => GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == GATING_A::PASS
    }
}
#[doc = "Field `gating` writer - Gating Clock"]
pub type GATING_W<'a, REG> = crate::BitWriter<'a, REG, GATING_A>;
impl<'a, REG> GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::PASS)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gating_field(&self) -> GATING_FIELD_R {
        GATING_FIELD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gating_field(&mut self) -> GATING_FIELD_W<RISCV_GATING_SPEC> {
        GATING_FIELD_W::new(self, 0)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<RISCV_GATING_SPEC> {
        GATING_W::new(self, 31)
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
#[doc = "RISC-V GATING Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_gating::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_gating::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISCV_GATING_SPEC;
impl crate::RegisterSpec for RISCV_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riscv_gating::R`](R) reader structure"]
impl crate::Readable for RISCV_GATING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`riscv_gating::W`](W) writer structure"]
impl crate::Writable for RISCV_GATING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_gating to value 0"]
impl crate::Resettable for RISCV_GATING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
