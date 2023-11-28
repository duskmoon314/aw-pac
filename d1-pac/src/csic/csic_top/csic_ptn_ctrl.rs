#[doc = "Register `csic_ptn_ctrl` reader"]
pub type R = crate::R<CSIC_PTN_CTRL_SPEC>;
#[doc = "Register `csic_ptn_ctrl` writer"]
pub type W = crate::W<CSIC_PTN_CTRL_SPEC>;
#[doc = "Field `ptn_gen_dly` reader - Clocks delayed before pattern generating start."]
pub type PTN_GEN_DLY_R = crate::FieldReader;
#[doc = "Field `ptn_gen_dly` writer - Clocks delayed before pattern generating start."]
pub type PTN_GEN_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ptn_gen_clk_div` reader - Packet generator clock divider"]
pub type PTN_GEN_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `ptn_gen_clk_div` writer - Packet generator clock divider"]
pub type PTN_GEN_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ptn_mode` reader - Pattern mode selection"]
pub type PTN_MODE_R = crate::FieldReader<PTN_MODE_A>;
#[doc = "Pattern mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTN_MODE_A {
    #[doc = "4: `100`"]
    NCSIC_YUV_8BIT = 4,
    #[doc = "5: `101`"]
    NCSIC_YUV_16BIT = 5,
    #[doc = "9: `1001`"]
    BT656_8BIT = 9,
    #[doc = "10: `1010`"]
    BT656_16BIT = 10,
    #[doc = "13: `1101`"]
    BAYER_12BIT = 13,
    #[doc = "14: `1110`"]
    UYVY422_12BIT = 14,
    #[doc = "15: `1111`"]
    UYVY420_12BIT = 15,
}
impl From<PTN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTN_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTN_MODE_A {
    type Ux = u8;
}
impl PTN_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTN_MODE_A> {
        match self.bits {
            4 => Some(PTN_MODE_A::NCSIC_YUV_8BIT),
            5 => Some(PTN_MODE_A::NCSIC_YUV_16BIT),
            9 => Some(PTN_MODE_A::BT656_8BIT),
            10 => Some(PTN_MODE_A::BT656_16BIT),
            13 => Some(PTN_MODE_A::BAYER_12BIT),
            14 => Some(PTN_MODE_A::UYVY422_12BIT),
            15 => Some(PTN_MODE_A::UYVY420_12BIT),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ncsic_yuv_8bit(&self) -> bool {
        *self == PTN_MODE_A::NCSIC_YUV_8BIT
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_ncsic_yuv_16bit(&self) -> bool {
        *self == PTN_MODE_A::NCSIC_YUV_16BIT
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_bt656_8bit(&self) -> bool {
        *self == PTN_MODE_A::BT656_8BIT
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_bt656_16bit(&self) -> bool {
        *self == PTN_MODE_A::BT656_16BIT
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_bayer_12bit(&self) -> bool {
        *self == PTN_MODE_A::BAYER_12BIT
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_uyvy422_12bit(&self) -> bool {
        *self == PTN_MODE_A::UYVY422_12BIT
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_uyvy420_12bit(&self) -> bool {
        *self == PTN_MODE_A::UYVY420_12BIT
    }
}
#[doc = "Field `ptn_mode` writer - Pattern mode selection"]
pub type PTN_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PTN_MODE_A>;
impl<'a, REG> PTN_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ncsic_yuv_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::NCSIC_YUV_8BIT)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ncsic_yuv_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::NCSIC_YUV_16BIT)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn bt656_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::BT656_8BIT)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn bt656_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::BT656_16BIT)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn bayer_12bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::BAYER_12BIT)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uyvy422_12bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::UYVY422_12BIT)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uyvy420_12bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_MODE_A::UYVY420_12BIT)
    }
}
#[doc = "Field `ptn_gen_data_width` reader - "]
pub type PTN_GEN_DATA_WIDTH_R = crate::FieldReader<PTN_GEN_DATA_WIDTH_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTN_GEN_DATA_WIDTH_A {
    #[doc = "0: `0`"]
    _8BIT = 0,
    #[doc = "1: `1`"]
    _10BIT = 1,
    #[doc = "2: `10`"]
    _12BIT = 2,
}
impl From<PTN_GEN_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: PTN_GEN_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTN_GEN_DATA_WIDTH_A {
    type Ux = u8;
}
impl PTN_GEN_DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTN_GEN_DATA_WIDTH_A> {
        match self.bits {
            0 => Some(PTN_GEN_DATA_WIDTH_A::_8BIT),
            1 => Some(PTN_GEN_DATA_WIDTH_A::_10BIT),
            2 => Some(PTN_GEN_DATA_WIDTH_A::_12BIT),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == PTN_GEN_DATA_WIDTH_A::_8BIT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == PTN_GEN_DATA_WIDTH_A::_10BIT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == PTN_GEN_DATA_WIDTH_A::_12BIT
    }
}
#[doc = "Field `ptn_gen_data_width` writer - "]
pub type PTN_GEN_DATA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PTN_GEN_DATA_WIDTH_A>;
impl<'a, REG> PTN_GEN_DATA_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_GEN_DATA_WIDTH_A::_8BIT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_GEN_DATA_WIDTH_A::_10BIT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_GEN_DATA_WIDTH_A::_12BIT)
    }
}
#[doc = "Field `ptn_port_sel` reader - Pattern Generator output port selection"]
pub type PTN_PORT_SEL_R = crate::FieldReader<PTN_PORT_SEL_A>;
#[doc = "Pattern Generator output port selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTN_PORT_SEL_A {
    #[doc = "2: `10`"]
    NSCIC0 = 2,
}
impl From<PTN_PORT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PTN_PORT_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTN_PORT_SEL_A {
    type Ux = u8;
}
impl PTN_PORT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTN_PORT_SEL_A> {
        match self.bits {
            2 => Some(PTN_PORT_SEL_A::NSCIC0),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_nscic0(&self) -> bool {
        *self == PTN_PORT_SEL_A::NSCIC0
    }
}
#[doc = "Field `ptn_port_sel` writer - Pattern Generator output port selection"]
pub type PTN_PORT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PTN_PORT_SEL_A>;
impl<'a, REG> PTN_PORT_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn nscic0(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_PORT_SEL_A::NSCIC0)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clocks delayed before pattern generating start."]
    #[inline(always)]
    pub fn ptn_gen_dly(&self) -> PTN_GEN_DLY_R {
        PTN_GEN_DLY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Packet generator clock divider"]
    #[inline(always)]
    pub fn ptn_gen_clk_div(&self) -> PTN_GEN_CLK_DIV_R {
        PTN_GEN_CLK_DIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Pattern mode selection"]
    #[inline(always)]
    pub fn ptn_mode(&self) -> PTN_MODE_R {
        PTN_MODE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn ptn_gen_data_width(&self) -> PTN_GEN_DATA_WIDTH_R {
        PTN_GEN_DATA_WIDTH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pattern Generator output port selection"]
    #[inline(always)]
    pub fn ptn_port_sel(&self) -> PTN_PORT_SEL_R {
        PTN_PORT_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clocks delayed before pattern generating start."]
    #[inline(always)]
    #[must_use]
    pub fn ptn_gen_dly(&mut self) -> PTN_GEN_DLY_W<CSIC_PTN_CTRL_SPEC> {
        PTN_GEN_DLY_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Packet generator clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn ptn_gen_clk_div(&mut self) -> PTN_GEN_CLK_DIV_W<CSIC_PTN_CTRL_SPEC> {
        PTN_GEN_CLK_DIV_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Pattern mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptn_mode(&mut self) -> PTN_MODE_W<CSIC_PTN_CTRL_SPEC> {
        PTN_MODE_W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn ptn_gen_data_width(&mut self) -> PTN_GEN_DATA_WIDTH_W<CSIC_PTN_CTRL_SPEC> {
        PTN_GEN_DATA_WIDTH_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Pattern Generator output port selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptn_port_sel(&mut self) -> PTN_PORT_SEL_W<CSIC_PTN_CTRL_SPEC> {
        PTN_PORT_SEL_W::new(self, 24)
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
#[doc = "CSIC Pattern Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PTN_CTRL_SPEC;
impl crate::RegisterSpec for CSIC_PTN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_ptn_ctrl::R`](R) reader structure"]
impl crate::Readable for CSIC_PTN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_ptn_ctrl::W`](W) writer structure"]
impl crate::Writable for CSIC_PTN_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_ptn_ctrl to value 0x0f"]
impl crate::Resettable for CSIC_PTN_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
