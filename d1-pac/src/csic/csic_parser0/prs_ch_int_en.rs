#[doc = "Register `prs_ch%s_int_en` reader"]
pub type R = crate::R<PRS_CH_INT_EN_SPEC>;
#[doc = "Register `prs_ch%s_int_en` writer"]
pub type W = crate::W<PRS_CH_INT_EN_SPEC>;
#[doc = "Field `input_para_int_en[0-1]` reader - "]
pub type INPUT_PARA_INT_EN_R = crate::BitReader<INPUT_PARA_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_PARA_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<INPUT_PARA_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_PARA_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_PARA_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INPUT_PARA_INT_EN_A {
        match self.bits {
            false => INPUT_PARA_INT_EN_A::DISABLE,
            true => INPUT_PARA_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INPUT_PARA_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INPUT_PARA_INT_EN_A::ENABLE
    }
}
#[doc = "Field `input_para_int_en[0-1]` writer - "]
pub type INPUT_PARA_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, INPUT_PARA_INT_EN_A>;
impl<'a, REG> INPUT_PARA_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT_PARA_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT_PARA_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `mul_err_int_en` reader - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
pub type MUL_ERR_INT_EN_R = crate::BitReader<MUL_ERR_INT_EN_A>;
#[doc = "Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUL_ERR_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<MUL_ERR_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MUL_ERR_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MUL_ERR_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUL_ERR_INT_EN_A {
        match self.bits {
            false => MUL_ERR_INT_EN_A::DISABLE,
            true => MUL_ERR_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MUL_ERR_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MUL_ERR_INT_EN_A::ENABLE
    }
}
#[doc = "Field `mul_err_int_en` writer - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
pub type MUL_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, MUL_ERR_INT_EN_A>;
impl<'a, REG> MUL_ERR_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MUL_ERR_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MUL_ERR_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `input_para0_int_en` field"]
    #[inline(always)]
    pub fn input_para_int_en(&self, n: u8) -> INPUT_PARA_INT_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        INPUT_PARA_INT_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - input_para0_int_en"]
    #[inline(always)]
    pub fn input_para0_int_en(&self) -> INPUT_PARA_INT_EN_R {
        INPUT_PARA_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - input_para1_int_en"]
    #[inline(always)]
    pub fn input_para1_int_en(&self) -> INPUT_PARA_INT_EN_R {
        INPUT_PARA_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
    #[inline(always)]
    pub fn mul_err_int_en(&self) -> MUL_ERR_INT_EN_R {
        MUL_ERR_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `input_para0_int_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn input_para_int_en(&mut self, n: u8) -> INPUT_PARA_INT_EN_W<PRS_CH_INT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        INPUT_PARA_INT_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - input_para0_int_en"]
    #[inline(always)]
    #[must_use]
    pub fn input_para0_int_en(&mut self) -> INPUT_PARA_INT_EN_W<PRS_CH_INT_EN_SPEC> {
        INPUT_PARA_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - input_para1_int_en"]
    #[inline(always)]
    #[must_use]
    pub fn input_para1_int_en(&mut self) -> INPUT_PARA_INT_EN_W<PRS_CH_INT_EN_SPEC> {
        INPUT_PARA_INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
    #[inline(always)]
    #[must_use]
    pub fn mul_err_int_en(&mut self) -> MUL_ERR_INT_EN_W<PRS_CH_INT_EN_SPEC> {
        MUL_ERR_INT_EN_W::new(self, 2)
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
#[doc = "Parser Channel\\[i\\] Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH_INT_EN_SPEC;
impl crate::RegisterSpec for PRS_CH_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch_int_en::R`](R) reader structure"]
impl crate::Readable for PRS_CH_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prs_ch_int_en::W`](W) writer structure"]
impl crate::Writable for PRS_CH_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ch%s_int_en to value 0"]
impl crate::Resettable for PRS_CH_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
