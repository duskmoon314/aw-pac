#[doc = "Register `i2s_pcm_rxchmap0` reader"]
pub type R = crate::R<I2S_PCM_RXCHMAP0_SPEC>;
#[doc = "Register `i2s_pcm_rxchmap0` writer"]
pub type W = crate::W<I2S_PCM_RXCHMAP0_SPEC>;
#[doc = "Field `ch_map[12-15]` reader - RX Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_R = crate::FieldReader;
#[doc = "Field `ch_map[12-15]` writer - RX Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ch_select[12-15]` reader - RX Channel %s Select"]
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
#[doc = "Field `ch_select[12-15]` writer - RX Channel %s Select"]
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
    #[doc = "RX Channel [12-15] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch12_map` field"]
    #[inline(always)]
    pub fn ch_map(&self, n: u8) -> CH_MAP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_MAP_R::new(((self.bits >> (n * 8)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - RX Channel 12 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch12_map(&self) -> CH_MAP_R {
        CH_MAP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RX Channel 13 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch13_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RX Channel 14 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch14_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RX Channel 15 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch15_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "RX Channel [12-15] Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch12_select` field"]
    #[inline(always)]
    pub fn ch_select(&self, n: u8) -> CH_SELECT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_SELECT_R::new(((self.bits >> (n * 8 + 4)) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RX Channel 12 Select"]
    #[inline(always)]
    pub fn ch12_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - RX Channel 13 Select"]
    #[inline(always)]
    pub fn ch13_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RX Channel 14 Select"]
    #[inline(always)]
    pub fn ch14_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - RX Channel 15 Select"]
    #[inline(always)]
    pub fn ch15_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "RX Channel [12-15] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch12_map` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map(&mut self, n: u8) -> CH_MAP_W<I2S_PCM_RXCHMAP0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_MAP_W::new(self, n * 8)
    }
    #[doc = "Bits 0:3 - RX Channel 12 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch12_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_MAP_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - RX Channel 13 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch13_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_MAP_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - RX Channel 14 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch14_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_MAP_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - RX Channel 15 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch15_map(&mut self) -> CH_MAP_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_MAP_W::new(self, 24)
    }
    #[doc = "RX Channel [12-15] Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch12_select` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_select(&mut self, n: u8) -> CH_SELECT_W<I2S_PCM_RXCHMAP0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_SELECT_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:5 - RX Channel 12 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch12_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - RX Channel 13 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch13_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - RX Channel 14 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch14_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP0_SPEC> {
        CH_SELECT_W::new(self, 20)
    }
    #[doc = "Bits 28:29 - RX Channel 15 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch15_select(&mut self) -> CH_SELECT_W<I2S_PCM_RXCHMAP0_SPEC> {
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
#[doc = "I2S/PCM RX Channel Mapping Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchmap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchmap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_RXCHMAP0_SPEC;
impl crate::RegisterSpec for I2S_PCM_RXCHMAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_rxchmap0::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_RXCHMAP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_rxchmap0::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_RXCHMAP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_rxchmap0 to value 0"]
impl crate::Resettable for I2S_PCM_RXCHMAP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
