#[doc = "Register `tmr%s_ctrl` reader"]
pub type R = crate::R<TMR_CTRL_SPEC>;
#[doc = "Register `tmr%s_ctrl` writer"]
pub type W = crate::W<TMR_CTRL_SPEC>;
#[doc = "Field `tmr_en` reader - "]
pub type TMR_EN_R = crate::BitReader<TMR_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR_EN_A {
    #[doc = "0: `0`"]
    STOP_PAUSE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<TMR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR_EN_A {
        match self.bits {
            false => TMR_EN_A::STOP_PAUSE,
            true => TMR_EN_A::START,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stop_pause(&self) -> bool {
        *self == TMR_EN_A::STOP_PAUSE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == TMR_EN_A::START
    }
}
#[doc = "Field `tmr_en` writer - "]
pub type TMR_EN_W<'a, REG> = crate::BitWriter<'a, REG, TMR_EN_A>;
impl<'a, REG> TMR_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop_pause(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_EN_A::STOP_PAUSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_EN_A::START)
    }
}
#[doc = "Field `tmr_reload` reader - "]
pub type TMR_RELOAD_R = crate::BitReader<TMR_RELOAD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR_RELOAD_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RELOAD = 1,
}
impl From<TMR_RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: TMR_RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR_RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR_RELOAD_A {
        match self.bits {
            false => TMR_RELOAD_A::NO_EFFECT,
            true => TMR_RELOAD_A::RELOAD,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TMR_RELOAD_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == TMR_RELOAD_A::RELOAD
    }
}
#[doc = "Field `tmr_reload` writer - "]
pub type TMR_RELOAD_W<'a, REG> = crate::BitWriter<'a, REG, TMR_RELOAD_A>;
impl<'a, REG> TMR_RELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_RELOAD_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_RELOAD_A::RELOAD)
    }
}
#[doc = "Field `tmr_clk_src` reader - "]
pub type TMR_CLK_SRC_R = crate::FieldReader<TMR_CLK_SRC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR_CLK_SRC_A {
    #[doc = "0: `0`"]
    LOSC = 0,
    #[doc = "1: `1`"]
    OSC24_M = 1,
}
impl From<TMR_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR_CLK_SRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR_CLK_SRC_A {
    type Ux = u8;
}
impl TMR_CLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR_CLK_SRC_A> {
        match self.bits {
            0 => Some(TMR_CLK_SRC_A::LOSC),
            1 => Some(TMR_CLK_SRC_A::OSC24_M),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_losc(&self) -> bool {
        *self == TMR_CLK_SRC_A::LOSC
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_osc24_m(&self) -> bool {
        *self == TMR_CLK_SRC_A::OSC24_M
    }
}
#[doc = "Field `tmr_clk_src` writer - "]
pub type TMR_CLK_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TMR_CLK_SRC_A>;
impl<'a, REG> TMR_CLK_SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn losc(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_SRC_A::LOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn osc24_m(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_SRC_A::OSC24_M)
    }
}
#[doc = "Field `tmr_clk_pres` reader - "]
pub type TMR_CLK_PRES_R = crate::FieldReader<TMR_CLK_PRES_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR_CLK_PRES_A {
    #[doc = "0: `0`"]
    P1 = 0,
    #[doc = "1: `1`"]
    P2 = 1,
    #[doc = "2: `10`"]
    P4 = 2,
    #[doc = "3: `11`"]
    P8 = 3,
    #[doc = "4: `100`"]
    P16 = 4,
    #[doc = "5: `101`"]
    P32 = 5,
    #[doc = "6: `110`"]
    P64 = 6,
    #[doc = "7: `111`"]
    P128 = 7,
}
impl From<TMR_CLK_PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR_CLK_PRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR_CLK_PRES_A {
    type Ux = u8;
}
impl TMR_CLK_PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR_CLK_PRES_A {
        match self.bits {
            0 => TMR_CLK_PRES_A::P1,
            1 => TMR_CLK_PRES_A::P2,
            2 => TMR_CLK_PRES_A::P4,
            3 => TMR_CLK_PRES_A::P8,
            4 => TMR_CLK_PRES_A::P16,
            5 => TMR_CLK_PRES_A::P32,
            6 => TMR_CLK_PRES_A::P64,
            7 => TMR_CLK_PRES_A::P128,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == TMR_CLK_PRES_A::P1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == TMR_CLK_PRES_A::P2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == TMR_CLK_PRES_A::P4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        *self == TMR_CLK_PRES_A::P8
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_p16(&self) -> bool {
        *self == TMR_CLK_PRES_A::P16
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_p32(&self) -> bool {
        *self == TMR_CLK_PRES_A::P32
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_p64(&self) -> bool {
        *self == TMR_CLK_PRES_A::P64
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_p128(&self) -> bool {
        *self == TMR_CLK_PRES_A::P128
    }
}
#[doc = "Field `tmr_clk_pres` writer - "]
pub type TMR_CLK_PRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TMR_CLK_PRES_A>;
impl<'a, REG> TMR_CLK_PRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P8)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn p16(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P16)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn p32(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P32)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn p64(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P64)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn p128(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_CLK_PRES_A::P128)
    }
}
#[doc = "Field `tmr_mode` reader - "]
pub type TMR_MODE_R = crate::BitReader<TMR_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR_MODE_A {
    #[doc = "0: `0`"]
    PERIODIC = 0,
    #[doc = "1: `1`"]
    SINGLE_COUNTING = 1,
}
impl From<TMR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TMR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR_MODE_A {
        match self.bits {
            false => TMR_MODE_A::PERIODIC,
            true => TMR_MODE_A::SINGLE_COUNTING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == TMR_MODE_A::PERIODIC
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_single_counting(&self) -> bool {
        *self == TMR_MODE_A::SINGLE_COUNTING
    }
}
#[doc = "Field `tmr_mode` writer - "]
pub type TMR_MODE_W<'a, REG> = crate::BitWriter<'a, REG, TMR_MODE_A>;
impl<'a, REG> TMR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_MODE_A::PERIODIC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn single_counting(self) -> &'a mut crate::W<REG> {
        self.variant(TMR_MODE_A::SINGLE_COUNTING)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmr_reload(&self) -> TMR_RELOAD_R {
        TMR_RELOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tmr_clk_src(&self) -> TMR_CLK_SRC_R {
        TMR_CLK_SRC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmr_clk_pres(&self) -> TMR_CLK_PRES_R {
        TMR_CLK_PRES_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TMR_MODE_R {
        TMR_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_en(&mut self) -> TMR_EN_W<TMR_CTRL_SPEC> {
        TMR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_reload(&mut self) -> TMR_RELOAD_W<TMR_CTRL_SPEC> {
        TMR_RELOAD_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_clk_src(&mut self) -> TMR_CLK_SRC_W<TMR_CTRL_SPEC> {
        TMR_CLK_SRC_W::new(self, 2)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_clk_pres(&mut self) -> TMR_CLK_PRES_W<TMR_CTRL_SPEC> {
        TMR_CLK_PRES_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_mode(&mut self) -> TMR_MODE_W<TMR_CTRL_SPEC> {
        TMR_MODE_W::new(self, 7)
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
#[doc = "Timer IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMR_CTRL_SPEC;
impl crate::RegisterSpec for TMR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr_ctrl::R`](R) reader structure"]
impl crate::Readable for TMR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmr_ctrl::W`](W) writer structure"]
impl crate::Writable for TMR_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tmr%s_ctrl to value 0"]
impl crate::Resettable for TMR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
