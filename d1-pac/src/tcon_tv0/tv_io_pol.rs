#[doc = "Register `tv_io_pol` reader"]
pub type R = crate::R<TV_IO_POL_SPEC>;
#[doc = "Register `tv_io_pol` writer"]
pub type W = crate::W<TV_IO_POL_SPEC>;
#[doc = "Field `io_inv[0-3]` reader - IO\\[i\\] Invert"]
pub type IO_INV_R = crate::BitReader<IO_INV_A>;
#[doc = "IO\\[i\\] Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_INV_A {
    #[doc = "0: Not invert"]
    NOT_INVERT = 0,
    #[doc = "1: Invert"]
    INVERT = 1,
}
impl From<IO_INV_A> for bool {
    #[inline(always)]
    fn from(variant: IO_INV_A) -> Self {
        variant as u8 != 0
    }
}
impl IO_INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IO_INV_A {
        match self.bits {
            false => IO_INV_A::NOT_INVERT,
            true => IO_INV_A::INVERT,
        }
    }
    #[doc = "Not invert"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == IO_INV_A::NOT_INVERT
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == IO_INV_A::INVERT
    }
}
#[doc = "Field `io_inv[0-3]` writer - IO\\[i\\] Invert"]
pub type IO_INV_W<'a, REG> = crate::BitWriter<'a, REG, IO_INV_A>;
impl<'a, REG> IO_INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not invert"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut crate::W<REG> {
        self.variant(IO_INV_A::NOT_INVERT)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(IO_INV_A::INVERT)
    }
}
impl R {
    #[doc = "IO\\[i\\] Invert\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `io0_inv` field"]
    #[inline(always)]
    pub fn io_inv(&self, n: u8) -> IO_INV_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        IO_INV_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io0_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io1_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io2_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io3_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "IO\\[i\\] Invert\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `io0_inv` field"]
    #[inline(always)]
    #[must_use]
    pub fn io_inv(&mut self, n: u8) -> IO_INV_W<TV_IO_POL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        IO_INV_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io0_inv(&mut self) -> IO_INV_W<TV_IO_POL_SPEC> {
        IO_INV_W::new(self, 24)
    }
    #[doc = "Bit 25 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io1_inv(&mut self) -> IO_INV_W<TV_IO_POL_SPEC> {
        IO_INV_W::new(self, 25)
    }
    #[doc = "Bit 26 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io2_inv(&mut self) -> IO_INV_W<TV_IO_POL_SPEC> {
        IO_INV_W::new(self, 26)
    }
    #[doc = "Bit 27 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io3_inv(&mut self) -> IO_INV_W<TV_IO_POL_SPEC> {
        IO_INV_W::new(self, 27)
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
#[doc = "TV SYNC Signal Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_io_pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_io_pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_IO_POL_SPEC;
impl crate::RegisterSpec for TV_IO_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_io_pol::R`](R) reader structure"]
impl crate::Readable for TV_IO_POL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_io_pol::W`](W) writer structure"]
impl crate::Writable for TV_IO_POL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_io_pol to value 0"]
impl crate::Resettable for TV_IO_POL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
