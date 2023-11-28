#[doc = "Register `rxdma_ctrl` reader"]
pub type R = crate::R<RXDMA_CTRL_SPEC>;
#[doc = "Register `rxdma_ctrl` writer"]
pub type W = crate::W<RXDMA_CTRL_SPEC>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::ENABLE)
    }
}
#[doc = "Field `mode` reader - "]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    CONTINOUS = 0,
    #[doc = "1: `1`"]
    LIMITED = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::CONTINOUS,
            true => MODE_A::LIMITED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        *self == MODE_A::CONTINOUS
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == MODE_A::LIMITED
    }
}
#[doc = "Field `mode` writer - "]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn continous(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CONTINOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::LIMITED)
    }
}
#[doc = "Field `blk_size` reader - "]
pub type BLK_SIZE_R = crate::FieldReader<BLK_SIZE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLK_SIZE_A {
    #[doc = "0: `0`"]
    B8 = 0,
    #[doc = "1: `1`"]
    B16 = 1,
    #[doc = "2: `10`"]
    B32 = 2,
    #[doc = "3: `11`"]
    B64 = 3,
}
impl From<BLK_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLK_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLK_SIZE_A {
    type Ux = u8;
}
impl BLK_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLK_SIZE_A {
        match self.bits {
            0 => BLK_SIZE_A::B8,
            1 => BLK_SIZE_A::B16,
            2 => BLK_SIZE_A::B32,
            3 => BLK_SIZE_A::B64,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == BLK_SIZE_A::B8
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        *self == BLK_SIZE_A::B16
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        *self == BLK_SIZE_A::B32
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        *self == BLK_SIZE_A::B64
    }
}
#[doc = "Field `blk_size` writer - "]
pub type BLK_SIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BLK_SIZE_A>;
impl<'a, REG> BLK_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn b8(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_SIZE_A::B8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn b16(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_SIZE_A::B16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn b32(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_SIZE_A::B32)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn b64(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_SIZE_A::B64)
    }
}
#[doc = "Field `ahb_burst_mode` reader - Set for AHB port burst supported"]
pub type AHB_BURST_MODE_R = crate::FieldReader<AHB_BURST_MODE_A>;
#[doc = "Set for AHB port burst supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHB_BURST_MODE_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    INCR4 = 1,
    #[doc = "2: `10`"]
    INCR8 = 2,
    #[doc = "3: `11`"]
    INCR16 = 3,
}
impl From<AHB_BURST_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_BURST_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AHB_BURST_MODE_A {
    type Ux = u8;
}
impl AHB_BURST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB_BURST_MODE_A {
        match self.bits {
            0 => AHB_BURST_MODE_A::SINGLE,
            1 => AHB_BURST_MODE_A::INCR4,
            2 => AHB_BURST_MODE_A::INCR8,
            3 => AHB_BURST_MODE_A::INCR16,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AHB_BURST_MODE_A::SINGLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == AHB_BURST_MODE_A::INCR4
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == AHB_BURST_MODE_A::INCR8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == AHB_BURST_MODE_A::INCR16
    }
}
#[doc = "Field `ahb_burst_mode` writer - Set for AHB port burst supported"]
pub type AHB_BURST_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AHB_BURST_MODE_A>;
impl<'a, REG> AHB_BURST_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(AHB_BURST_MODE_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(AHB_BURST_MODE_A::INCR4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(AHB_BURST_MODE_A::INCR8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(AHB_BURST_MODE_A::INCR16)
    }
}
#[doc = "Field `timeout_enable` reader - RXDMA Timeout Enable"]
pub type TIMEOUT_ENABLE_R = crate::BitReader;
#[doc = "Field `timeout_enable` writer - RXDMA Timeout Enable"]
pub type TIMEOUT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timeout_threshold` reader - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
pub type TIMEOUT_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `timeout_threshold` writer - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
pub type TIMEOUT_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blk_size(&self) -> BLK_SIZE_R {
        BLK_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set for AHB port burst supported"]
    #[inline(always)]
    pub fn ahb_burst_mode(&self) -> AHB_BURST_MODE_R {
        AHB_BURST_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RXDMA Timeout Enable"]
    #[inline(always)]
    pub fn timeout_enable(&self) -> TIMEOUT_ENABLE_R {
        TIMEOUT_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:23 - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
    #[inline(always)]
    pub fn timeout_threshold(&self) -> TIMEOUT_THRESHOLD_R {
        TIMEOUT_THRESHOLD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<RXDMA_CTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<RXDMA_CTRL_SPEC> {
        MODE_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn blk_size(&mut self) -> BLK_SIZE_W<RXDMA_CTRL_SPEC> {
        BLK_SIZE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Set for AHB port burst supported"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_burst_mode(&mut self) -> AHB_BURST_MODE_W<RXDMA_CTRL_SPEC> {
        AHB_BURST_MODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - RXDMA Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_enable(&mut self) -> TIMEOUT_ENABLE_W<RXDMA_CTRL_SPEC> {
        TIMEOUT_ENABLE_W::new(self, 6)
    }
    #[doc = "Bits 8:23 - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_threshold(&mut self) -> TIMEOUT_THRESHOLD_W<RXDMA_CTRL_SPEC> {
        TIMEOUT_THRESHOLD_W::new(self, 8)
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
#[doc = "UART RXDMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_CTRL_SPEC;
impl crate::RegisterSpec for RXDMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_ctrl::R`](R) reader structure"]
impl crate::Readable for RXDMA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_ctrl::W`](W) writer structure"]
impl crate::Writable for RXDMA_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_ctrl to value 0"]
impl crate::Resettable for RXDMA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
