#[doc = "Register `msgbox_bgr` reader"]
pub type R = crate::R<MSGBOX_BGR_SPEC>;
#[doc = "Register `msgbox_bgr` writer"]
pub type W = crate::W<MSGBOX_BGR_SPEC>;
#[doc = "Field `msgbox_gating[0-2]` reader - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
pub type MSGBOX_GATING_R = crate::BitReader<MSGBOX_GATING_A>;
#[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSGBOX_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<MSGBOX_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: MSGBOX_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl MSGBOX_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSGBOX_GATING_A {
        match self.bits {
            false => MSGBOX_GATING_A::MASK,
            true => MSGBOX_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MSGBOX_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == MSGBOX_GATING_A::PASS
    }
}
#[doc = "Field `msgbox_gating[0-2]` writer - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
pub type MSGBOX_GATING_W<'a, REG> = crate::BitWriter<'a, REG, MSGBOX_GATING_A>;
impl<'a, REG> MSGBOX_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(MSGBOX_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(MSGBOX_GATING_A::PASS)
    }
}
#[doc = "Field `msgbox_rst[0-2]` reader - CPU, DSP, RISC-V MSGBOX Reset"]
pub type MSGBOX_RST_R = crate::BitReader<MSGBOX_RST_A>;
#[doc = "CPU, DSP, RISC-V MSGBOX Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSGBOX_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<MSGBOX_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGBOX_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl MSGBOX_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSGBOX_RST_A {
        match self.bits {
            false => MSGBOX_RST_A::ASSERT,
            true => MSGBOX_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == MSGBOX_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == MSGBOX_RST_A::DEASSERT
    }
}
#[doc = "Field `msgbox_rst[0-2]` writer - CPU, DSP, RISC-V MSGBOX Reset"]
pub type MSGBOX_RST_W<'a, REG> = crate::BitWriter<'a, REG, MSGBOX_RST_A>;
impl<'a, REG> MSGBOX_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(MSGBOX_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(MSGBOX_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `msgbox0_gating` field"]
    #[inline(always)]
    pub fn msgbox_gating(&self, n: u8) -> MSGBOX_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        MSGBOX_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox0_gating(&self) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox1_gating(&self) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox2_gating(&self) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "CPU, DSP, RISC-V MSGBOX Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `msgbox0_rst` field"]
    #[inline(always)]
    pub fn msgbox_rst(&self, n: u8) -> MSGBOX_RST_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        MSGBOX_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox0_rst(&self) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox1_rst(&self) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox2_rst(&self) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `msgbox0_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox_gating(&mut self, n: u8) -> MSGBOX_GATING_W<MSGBOX_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        MSGBOX_GATING_W::new(self, n)
    }
    #[doc = "Bit 0 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox0_gating(&mut self) -> MSGBOX_GATING_W<MSGBOX_BGR_SPEC> {
        MSGBOX_GATING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox1_gating(&mut self) -> MSGBOX_GATING_W<MSGBOX_BGR_SPEC> {
        MSGBOX_GATING_W::new(self, 1)
    }
    #[doc = "Bit 2 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox2_gating(&mut self) -> MSGBOX_GATING_W<MSGBOX_BGR_SPEC> {
        MSGBOX_GATING_W::new(self, 2)
    }
    #[doc = "CPU, DSP, RISC-V MSGBOX Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `msgbox0_rst` field"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox_rst(&mut self, n: u8) -> MSGBOX_RST_W<MSGBOX_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        MSGBOX_RST_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox0_rst(&mut self) -> MSGBOX_RST_W<MSGBOX_BGR_SPEC> {
        MSGBOX_RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox1_rst(&mut self) -> MSGBOX_RST_W<MSGBOX_BGR_SPEC> {
        MSGBOX_RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox2_rst(&mut self) -> MSGBOX_RST_W<MSGBOX_BGR_SPEC> {
        MSGBOX_RST_W::new(self, 18)
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
#[doc = "MSGBOX Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_BGR_SPEC;
impl crate::RegisterSpec for MSGBOX_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_bgr::R`](R) reader structure"]
impl crate::Readable for MSGBOX_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_bgr::W`](W) writer structure"]
impl crate::Writable for MSGBOX_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_bgr to value 0"]
impl crate::Resettable for MSGBOX_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
