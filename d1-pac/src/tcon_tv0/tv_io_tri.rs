#[doc = "Register `tv_io_tri` reader"]
pub type R = crate::R<TV_IO_TRI_SPEC>;
#[doc = "Register `tv_io_tri` writer"]
pub type W = crate::W<TV_IO_TRI_SPEC>;
#[doc = "Field `io_output_tri_en[0-3]` reader - IO\\[i\\] Output Trigger Enable"]
pub type IO_OUTPUT_TRI_EN_R = crate::BitReader<IO_OUTPUT_TRI_EN_A>;
#[doc = "IO\\[i\\] Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_OUTPUT_TRI_EN_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<IO_OUTPUT_TRI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IO_OUTPUT_TRI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl IO_OUTPUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IO_OUTPUT_TRI_EN_A {
        match self.bits {
            true => IO_OUTPUT_TRI_EN_A::DISABLE,
            false => IO_OUTPUT_TRI_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IO_OUTPUT_TRI_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IO_OUTPUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `io_output_tri_en[0-3]` writer - IO\\[i\\] Output Trigger Enable"]
pub type IO_OUTPUT_TRI_EN_W<'a, REG> = crate::BitWriter<'a, REG, IO_OUTPUT_TRI_EN_A>;
impl<'a, REG> IO_OUTPUT_TRI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IO_OUTPUT_TRI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IO_OUTPUT_TRI_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "IO\\[i\\] Output Trigger Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `io0_output_tri_en` field"]
    #[inline(always)]
    pub fn io_output_tri_en(&self, n: u8) -> IO_OUTPUT_TRI_EN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io0_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io1_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io2_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io3_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "IO\\[i\\] Output Trigger Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `io0_output_tri_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn io_output_tri_en(&mut self, n: u8) -> IO_OUTPUT_TRI_EN_W<TV_IO_TRI_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        IO_OUTPUT_TRI_EN_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io0_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<TV_IO_TRI_SPEC> {
        IO_OUTPUT_TRI_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io1_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<TV_IO_TRI_SPEC> {
        IO_OUTPUT_TRI_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io2_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<TV_IO_TRI_SPEC> {
        IO_OUTPUT_TRI_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io3_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<TV_IO_TRI_SPEC> {
        IO_OUTPUT_TRI_EN_W::new(self, 27)
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
#[doc = "TV SYNC Signal IO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_io_tri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_io_tri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_IO_TRI_SPEC;
impl crate::RegisterSpec for TV_IO_TRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_io_tri::R`](R) reader structure"]
impl crate::Readable for TV_IO_TRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_io_tri::W`](W) writer structure"]
impl crate::Writable for TV_IO_TRI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_io_tri to value 0"]
impl crate::Resettable for TV_IO_TRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
