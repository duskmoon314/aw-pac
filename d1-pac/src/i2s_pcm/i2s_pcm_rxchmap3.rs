#[doc = "Register `i2s_pcm_rxchmap3` reader"]
pub type R = crate::R<I2S_PCM_RXCHMAP3_SPEC>;
#[doc = "Register `i2s_pcm_rxchmap3` writer"]
pub type W = crate::W<I2S_PCM_RXCHMAP3_SPEC>;
#[doc = "Field `ch_map[0-3]` reader - RX Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_R = crate::FieldReader;
#[doc = "Field `ch_map[0-3]` writer - RX Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ch_select[0-3]` reader - RX Channel %s Select"]
pub type CH_SELECT_R = crate::FieldReader<CH_SELECT_A>;
#[doc = "RX Channel %s Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_SELECT_A {
    #[doc = "0: `0`"]
    SDI0 = 0,
    #[doc = "1: `1`"]
    SDI1 = 1,
    #[doc = "2: `10`"]
    SDI2 = 2,
    #[doc = "3: `11`"]
    SDI3 = 3,
}
impl From<CH_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH_SELECT_A {
    type Ux = u8;
}
impl CH_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH_SELECT_A {
        match self.bits {
            0 => CH_SELECT_A::SDI0,
            1 => CH_SELECT_A::SDI1,
            2 => CH_SELECT_A::SDI2,
            3 => CH_SELECT_A::SDI3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sdi0(&self) -> bool {
        *self == CH_SELECT_A::SDI0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdi1(&self) -> bool {
        *self == CH_SELECT_A::SDI1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sdi2(&self) -> bool {
        *self == CH_SELECT_A::SDI2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_sdi3(&self) -> bool {
        *self == CH_SELECT_A::SDI3
    }
}
#[doc = "Field `ch_select[0-3]` writer - RX Channel %s Select"]
pub type CH_SELECT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH_SELECT_A>;
impl<'a, REG> CH_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sdi0(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SELECT_A::SDI0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdi1(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SELECT_A::SDI1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdi2(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SELECT_A::SDI2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdi3(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SELECT_A::SDI3)
    }
}
impl R {
    #[doc = "RX Channel [0-3] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_map` field"]
    #[inline(always)]
    pub fn ch_map(&self, n: u8) -> CH_MAP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_MAP_R::new(((self.bits >> (n * 8)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - RX Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch0_map(&self) -> CH_MAP_R {
        CH_MAP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RX Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch1_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RX Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch2_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RX Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch3_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "RX Channel [0-3] Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_select` field"]
    #[inline(always)]
    pub fn ch_select(&self, n: u8) -> CH_SELECT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_SELECT_R::new(((self.bits >> (n * 8 + 4)) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RX Channel 0 Select"]
    #[inline(always)]
    pub fn ch0_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - RX Channel 1 Select"]
    #[inline(always)]
    pub fn ch1_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RX Channel 2 Select"]
    #[inline(always)]
    pub fn ch2_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - RX Channel 3 Select"]
    #[inline(always)]
    pub fn ch3_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "RX Channel [0-3] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_map` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map(&mut self, n: u8) -> CH_MAP_W<I2S_PCM_RXCHMAP3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_MAP_W::new(self, n * 8)
    }
    #[doc = "Bits 0:3 - RX Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_MAP_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - RX Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_MAP_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - RX Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_MAP_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - RX Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_MAP_W::new(self, 24)
    }
    #[doc = "RX Channel [0-3] Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_select` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_select(&mut self, n: u8) -> CH_SELECT_W<I2S_PCM_RXCHMAP3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_SELECT_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:5 - RX Channel 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - RX Channel 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - RX Channel 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_SELECT_W::new(self, 20)
    }
    #[doc = "Bits 28:29 - RX Channel 3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP3_SPEC> {
        CH_SELECT_W::new(self, 28)
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
#[doc = "I2S/PCM RX Channel Mapping Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchmap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchmap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_RXCHMAP3_SPEC;
impl crate::RegisterSpec for I2S_PCM_RXCHMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_rxchmap3::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_RXCHMAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_rxchmap3::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_RXCHMAP3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_rxchmap3 to value 0"]
impl crate::Resettable for I2S_PCM_RXCHMAP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
