#[doc = "Register `usb_bgr` reader"]
pub type R = crate::R<USB_BGR_SPEC>;
#[doc = "Register `usb_bgr` writer"]
pub type W = crate::W<USB_BGR_SPEC>;
#[doc = "Field `usbohci_gating[0-1]` reader - USBOHCI Gating Clock"]
pub type USBOHCI_GATING_R = crate::BitReader<USBOHCI_GATING_A>;
#[doc = "USBOHCI Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOHCI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<USBOHCI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: USBOHCI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl USBOHCI_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBOHCI_GATING_A {
        match self.bits {
            false => USBOHCI_GATING_A::MASK,
            true => USBOHCI_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == USBOHCI_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == USBOHCI_GATING_A::PASS
    }
}
#[doc = "Field `usbohci_gating[0-1]` writer - USBOHCI Gating Clock"]
pub type USBOHCI_GATING_W<'a, REG> = crate::BitWriter<'a, REG, USBOHCI_GATING_A>;
impl<'a, REG> USBOHCI_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(USBOHCI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(USBOHCI_GATING_A::PASS)
    }
}
#[doc = "Field `usbehci_gating[0-1]` reader - USBEHCI Gating Clock"]
pub type USBEHCI_GATING_R = crate::BitReader<USBEHCI_GATING_A>;
#[doc = "USBEHCI Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBEHCI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<USBEHCI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: USBEHCI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl USBEHCI_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBEHCI_GATING_A {
        match self.bits {
            false => USBEHCI_GATING_A::MASK,
            true => USBEHCI_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == USBEHCI_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == USBEHCI_GATING_A::PASS
    }
}
#[doc = "Field `usbehci_gating[0-1]` writer - USBEHCI Gating Clock"]
pub type USBEHCI_GATING_W<'a, REG> = crate::BitWriter<'a, REG, USBEHCI_GATING_A>;
impl<'a, REG> USBEHCI_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(USBEHCI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(USBEHCI_GATING_A::PASS)
    }
}
#[doc = "Field `usbotg0_gating` reader - USBOTG0 Gating Clock"]
pub type USBOTG0_GATING_R = crate::BitReader<USBOTG0_GATING_A>;
#[doc = "USBOTG0 Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTG0_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<USBOTG0_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTG0_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl USBOTG0_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBOTG0_GATING_A {
        match self.bits {
            false => USBOTG0_GATING_A::MASK,
            true => USBOTG0_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == USBOTG0_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == USBOTG0_GATING_A::PASS
    }
}
#[doc = "Field `usbotg0_gating` writer - USBOTG0 Gating Clock"]
pub type USBOTG0_GATING_W<'a, REG> = crate::BitWriter<'a, REG, USBOTG0_GATING_A>;
impl<'a, REG> USBOTG0_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTG0_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTG0_GATING_A::PASS)
    }
}
#[doc = "Field `usbohci_rst[0-1]` reader - USBOHCI Reset"]
pub type USBOHCI_RST_R = crate::BitReader<USBOHCI_RST_A>;
#[doc = "USBOHCI Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOHCI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<USBOHCI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USBOHCI_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBOHCI_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBOHCI_RST_A {
        match self.bits {
            false => USBOHCI_RST_A::ASSERT,
            true => USBOHCI_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == USBOHCI_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == USBOHCI_RST_A::DEASSERT
    }
}
#[doc = "Field `usbohci_rst[0-1]` writer - USBOHCI Reset"]
pub type USBOHCI_RST_W<'a, REG> = crate::BitWriter<'a, REG, USBOHCI_RST_A>;
impl<'a, REG> USBOHCI_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(USBOHCI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(USBOHCI_RST_A::DEASSERT)
    }
}
#[doc = "Field `usbehci_rst[0-1]` reader - USBEHCI Reset"]
pub type USBEHCI_RST_R = crate::BitReader<USBEHCI_RST_A>;
#[doc = "USBEHCI Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBEHCI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<USBEHCI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USBEHCI_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBEHCI_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBEHCI_RST_A {
        match self.bits {
            false => USBEHCI_RST_A::ASSERT,
            true => USBEHCI_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == USBEHCI_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == USBEHCI_RST_A::DEASSERT
    }
}
#[doc = "Field `usbehci_rst[0-1]` writer - USBEHCI Reset"]
pub type USBEHCI_RST_W<'a, REG> = crate::BitWriter<'a, REG, USBEHCI_RST_A>;
impl<'a, REG> USBEHCI_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(USBEHCI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(USBEHCI_RST_A::DEASSERT)
    }
}
#[doc = "Field `usbotg0_rst` reader - USBOTG0 Reset"]
pub type USBOTG0_RST_R = crate::BitReader<USBOTG0_RST_A>;
#[doc = "USBOTG0 Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTG0_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<USBOTG0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTG0_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBOTG0_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBOTG0_RST_A {
        match self.bits {
            false => USBOTG0_RST_A::ASSERT,
            true => USBOTG0_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == USBOTG0_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == USBOTG0_RST_A::DEASSERT
    }
}
#[doc = "Field `usbotg0_rst` writer - USBOTG0 Reset"]
pub type USBOTG0_RST_W<'a, REG> = crate::BitWriter<'a, REG, USBOTG0_RST_A>;
impl<'a, REG> USBOTG0_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTG0_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTG0_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "USBOHCI Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbohci0_gating` field"]
    #[inline(always)]
    pub fn usbohci_gating(&self, n: u8) -> USBOHCI_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBOHCI_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - USBOHCI Gating Clock"]
    #[inline(always)]
    pub fn usbohci0_gating(&self) -> USBOHCI_GATING_R {
        USBOHCI_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBOHCI Gating Clock"]
    #[inline(always)]
    pub fn usbohci1_gating(&self) -> USBOHCI_GATING_R {
        USBOHCI_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "USBEHCI Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbehci0_gating` field"]
    #[inline(always)]
    pub fn usbehci_gating(&self, n: u8) -> USBEHCI_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBEHCI_GATING_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - USBEHCI Gating Clock"]
    #[inline(always)]
    pub fn usbehci0_gating(&self) -> USBEHCI_GATING_R {
        USBEHCI_GATING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USBEHCI Gating Clock"]
    #[inline(always)]
    pub fn usbehci1_gating(&self) -> USBEHCI_GATING_R {
        USBEHCI_GATING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBOTG0 Gating Clock"]
    #[inline(always)]
    pub fn usbotg0_gating(&self) -> USBOTG0_GATING_R {
        USBOTG0_GATING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "USBOHCI Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbohci0_rst` field"]
    #[inline(always)]
    pub fn usbohci_rst(&self, n: u8) -> USBOHCI_RST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBOHCI_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - USBOHCI Reset"]
    #[inline(always)]
    pub fn usbohci0_rst(&self) -> USBOHCI_RST_R {
        USBOHCI_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USBOHCI Reset"]
    #[inline(always)]
    pub fn usbohci1_rst(&self) -> USBOHCI_RST_R {
        USBOHCI_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "USBEHCI Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbehci0_rst` field"]
    #[inline(always)]
    pub fn usbehci_rst(&self, n: u8) -> USBEHCI_RST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBEHCI_RST_R::new(((self.bits >> (n + 20)) & 1) != 0)
    }
    #[doc = "Bit 20 - USBEHCI Reset"]
    #[inline(always)]
    pub fn usbehci0_rst(&self) -> USBEHCI_RST_R {
        USBEHCI_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USBEHCI Reset"]
    #[inline(always)]
    pub fn usbehci1_rst(&self) -> USBEHCI_RST_R {
        USBEHCI_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - USBOTG0 Reset"]
    #[inline(always)]
    pub fn usbotg0_rst(&self) -> USBOTG0_RST_R {
        USBOTG0_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "USBOHCI Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbohci0_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn usbohci_gating(&mut self, n: u8) -> USBOHCI_GATING_W<USB_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBOHCI_GATING_W::new(self, n)
    }
    #[doc = "Bit 0 - USBOHCI Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usbohci0_gating(&mut self) -> USBOHCI_GATING_W<USB_BGR_SPEC> {
        USBOHCI_GATING_W::new(self, 0)
    }
    #[doc = "Bit 1 - USBOHCI Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usbohci1_gating(&mut self) -> USBOHCI_GATING_W<USB_BGR_SPEC> {
        USBOHCI_GATING_W::new(self, 1)
    }
    #[doc = "USBEHCI Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbehci0_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn usbehci_gating(&mut self, n: u8) -> USBEHCI_GATING_W<USB_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBEHCI_GATING_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - USBEHCI Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usbehci0_gating(&mut self) -> USBEHCI_GATING_W<USB_BGR_SPEC> {
        USBEHCI_GATING_W::new(self, 4)
    }
    #[doc = "Bit 5 - USBEHCI Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usbehci1_gating(&mut self) -> USBEHCI_GATING_W<USB_BGR_SPEC> {
        USBEHCI_GATING_W::new(self, 5)
    }
    #[doc = "Bit 8 - USBOTG0 Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usbotg0_gating(&mut self) -> USBOTG0_GATING_W<USB_BGR_SPEC> {
        USBOTG0_GATING_W::new(self, 8)
    }
    #[doc = "USBOHCI Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbohci0_rst` field"]
    #[inline(always)]
    #[must_use]
    pub fn usbohci_rst(&mut self, n: u8) -> USBOHCI_RST_W<USB_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBOHCI_RST_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - USBOHCI Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbohci0_rst(&mut self) -> USBOHCI_RST_W<USB_BGR_SPEC> {
        USBOHCI_RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - USBOHCI Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbohci1_rst(&mut self) -> USBOHCI_RST_W<USB_BGR_SPEC> {
        USBOHCI_RST_W::new(self, 17)
    }
    #[doc = "USBEHCI Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `usbehci0_rst` field"]
    #[inline(always)]
    #[must_use]
    pub fn usbehci_rst(&mut self, n: u8) -> USBEHCI_RST_W<USB_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USBEHCI_RST_W::new(self, n + 20)
    }
    #[doc = "Bit 20 - USBEHCI Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbehci0_rst(&mut self) -> USBEHCI_RST_W<USB_BGR_SPEC> {
        USBEHCI_RST_W::new(self, 20)
    }
    #[doc = "Bit 21 - USBEHCI Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbehci1_rst(&mut self) -> USBEHCI_RST_W<USB_BGR_SPEC> {
        USBEHCI_RST_W::new(self, 21)
    }
    #[doc = "Bit 24 - USBOTG0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbotg0_rst(&mut self) -> USBOTG0_RST_W<USB_BGR_SPEC> {
        USBOTG0_RST_W::new(self, 24)
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
#[doc = "USB Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_BGR_SPEC;
impl crate::RegisterSpec for USB_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_bgr::R`](R) reader structure"]
impl crate::Readable for USB_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_bgr::W`](W) writer structure"]
impl crate::Writable for USB_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_bgr to value 0"]
impl crate::Resettable for USB_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
