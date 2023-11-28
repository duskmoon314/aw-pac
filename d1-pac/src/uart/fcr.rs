#[doc = "Register `fcr` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `fifoe` writer - "]
pub type FIFOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfifor` writer - "]
pub type RFIFOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xfifor` writer - "]
pub type XFIFOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAM_AW {
    #[doc = "0: `0`"]
    MODE_0 = 0,
    #[doc = "1: `1`"]
    MODE_1 = 1,
}
impl From<DMAM_AW> for bool {
    #[inline(always)]
    fn from(variant: DMAM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dmam` writer - "]
pub type DMAM_W<'a, REG> = crate::BitWriter<'a, REG, DMAM_AW>;
impl<'a, REG> DMAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAM_AW::MODE_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAM_AW::MODE_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFT_AW {
    #[doc = "0: `0`"]
    EMPTY = 0,
    #[doc = "1: `1`"]
    TWO_CHARACTERS = 1,
    #[doc = "2: `10`"]
    QUARTER_FULL = 2,
    #[doc = "3: `11`"]
    HALF_FULL = 3,
}
impl From<TFT_AW> for u8 {
    #[inline(always)]
    fn from(variant: TFT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFT_AW {
    type Ux = u8;
}
#[doc = "Field `tft` writer - "]
pub type TFT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TFT_AW>;
impl<'a, REG> TFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TFT_AW::EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_characters(self) -> &'a mut crate::W<REG> {
        self.variant(TFT_AW::TWO_CHARACTERS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut crate::W<REG> {
        self.variant(TFT_AW::QUARTER_FULL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut crate::W<REG> {
        self.variant(TFT_AW::HALF_FULL)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT_AW {
    #[doc = "0: `0`"]
    ONE_CHARACTER = 0,
    #[doc = "1: `1`"]
    QUARTER_FULL = 1,
    #[doc = "2: `10`"]
    HALF_FULL = 2,
    #[doc = "3: `11`"]
    TWO_LESS_THAN_FULL = 3,
}
impl From<RT_AW> for u8 {
    #[inline(always)]
    fn from(variant: RT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RT_AW {
    type Ux = u8;
}
#[doc = "Field `rt` writer - "]
pub type RT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RT_AW>;
impl<'a, REG> RT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_character(self) -> &'a mut crate::W<REG> {
        self.variant(RT_AW::ONE_CHARACTER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut crate::W<REG> {
        self.variant(RT_AW::QUARTER_FULL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut crate::W<REG> {
        self.variant(RT_AW::HALF_FULL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn two_less_than_full(self) -> &'a mut crate::W<REG> {
        self.variant(RT_AW::TWO_LESS_THAN_FULL)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifoe(&mut self) -> FIFOE_W<FCR_SPEC> {
        FIFOE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rfifor(&mut self) -> RFIFOR_W<FCR_SPEC> {
        RFIFOR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xfifor(&mut self) -> XFIFOR_W<FCR_SPEC> {
        XFIFOR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dmam(&mut self) -> DMAM_W<FCR_SPEC> {
        DMAM_W::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn tft(&mut self) -> TFT_W<FCR_SPEC> {
        TFT_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RT_W<FCR_SPEC> {
        RT_W::new(self, 6)
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
#[doc = "UART FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fcr to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
