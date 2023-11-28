#[doc = "Register `dac_vol_ctrl` reader"]
pub type R = crate::R<DAC_VOL_CTRL_SPEC>;
#[doc = "Register `dac_vol_ctrl` writer"]
pub type W = crate::W<DAC_VOL_CTRL_SPEC>;
#[doc = "Field `dac_vol_r` reader - DAC right channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
pub type DAC_VOL_R_R = crate::FieldReader;
#[doc = "Field `dac_vol_r` writer - DAC right channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
pub type DAC_VOL_R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dac_vol_l` reader - DAC left channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
pub type DAC_VOL_L_R = crate::FieldReader;
#[doc = "Field `dac_vol_l` writer - DAC left channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
pub type DAC_VOL_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dac_vol_sel` reader - DAC Volume Control Selection Enable"]
pub type DAC_VOL_SEL_R = crate::BitReader<DAC_VOL_SEL_A>;
#[doc = "DAC Volume Control Selection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_VOL_SEL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DAC_VOL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_VOL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_VOL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_VOL_SEL_A {
        match self.bits {
            false => DAC_VOL_SEL_A::DISABLE,
            true => DAC_VOL_SEL_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DAC_VOL_SEL_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DAC_VOL_SEL_A::ENABLE
    }
}
#[doc = "Field `dac_vol_sel` writer - DAC Volume Control Selection Enable"]
pub type DAC_VOL_SEL_W<'a, REG> = crate::BitWriter<'a, REG, DAC_VOL_SEL_A>;
impl<'a, REG> DAC_VOL_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_VOL_SEL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_VOL_SEL_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC right channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
    #[inline(always)]
    pub fn dac_vol_r(&self) -> DAC_VOL_R_R {
        DAC_VOL_R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC left channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
    #[inline(always)]
    pub fn dac_vol_l(&self) -> DAC_VOL_L_R {
        DAC_VOL_L_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - DAC Volume Control Selection Enable"]
    #[inline(always)]
    pub fn dac_vol_sel(&self) -> DAC_VOL_SEL_R {
        DAC_VOL_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC right channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
    #[inline(always)]
    #[must_use]
    pub fn dac_vol_r(&mut self) -> DAC_VOL_R_W<DAC_VOL_CTRL_SPEC> {
        DAC_VOL_R_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC left channel volum\n\n(-119.25 dB to 71.25 dB, 0.75 dB/Step)\n\n0x00: Mute\n\n0x01: -119.25 dB\n\n...\n\n0x9F = -0.75 dB\n\n0xA0 = 0 dB\n\n0xA1 = 0.75 dB\n\n...\n\n0xFF = 71.25 dBe"]
    #[inline(always)]
    #[must_use]
    pub fn dac_vol_l(&mut self) -> DAC_VOL_L_W<DAC_VOL_CTRL_SPEC> {
        DAC_VOL_L_W::new(self, 8)
    }
    #[doc = "Bit 16 - DAC Volume Control Selection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_vol_sel(&mut self) -> DAC_VOL_SEL_W<DAC_VOL_CTRL_SPEC> {
        DAC_VOL_SEL_W::new(self, 16)
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
#[doc = "DAC Volume Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_vol_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_vol_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_VOL_CTRL_SPEC;
impl crate::RegisterSpec for DAC_VOL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_vol_ctrl::R`](R) reader structure"]
impl crate::Readable for DAC_VOL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac_vol_ctrl::W`](W) writer structure"]
impl crate::Writable for DAC_VOL_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dac_vol_ctrl to value 0"]
impl crate::Resettable for DAC_VOL_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
