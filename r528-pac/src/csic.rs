#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - CSIC_CCU"]
    pub csic_ccu: CSIC_CCU,
    _reserved1: [u8; 0x07f0],
    #[doc = "0x800..0x908 - CSIC_TOP"]
    pub csic_top: CSIC_TOP,
    _reserved2: [u8; 0x06f8],
    #[doc = "0x1000..0x151c - CSIC_PARSER0"]
    pub csic_parser0: CSIC_PARSER0,
    _reserved3: [u8; 0x7ae4],
    #[doc = "0x9000 - CSIC_DMA"]
    pub csic_dma: crate::ArrayProxy<CSIC_DMA, 2, 0x0200>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_CCU {
    #[doc = "0x00 - CCU Clock Mode Register"]
    pub ccu_clk_mode_reg: crate::Reg<self::csic_ccu::ccu_clk_mode_reg::CCU_CLK_MODE_REG_SPEC>,
    #[doc = "0x04 - CCU Parser Clock Enable Register"]
    pub ccu_parser_clk_en_reg:
        crate::Reg<self::csic_ccu::ccu_parser_clk_en_reg::CCU_PARSER_CLK_EN_REG_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - CCU Post0 Clock Enable Register"]
    pub ccu_post0_clk_en_reg:
        crate::Reg<self::csic_ccu::ccu_post0_clk_en_reg::CCU_POST0_CLK_EN_REG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CSIC_CCU"]
pub mod csic_ccu;
#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_TOP {
    #[doc = "0x00 - CSIC TOP Enable Register"]
    pub csic_top_en_reg: crate::Reg<self::csic_top::csic_top_en_reg::CSIC_TOP_EN_REG_SPEC>,
    #[doc = "0x04 - CSIC Pattern Generation Enable Register"]
    pub csic_ptn_gen_en_reg:
        crate::Reg<self::csic_top::csic_ptn_gen_en_reg::CSIC_PTN_GEN_EN_REG_SPEC>,
    #[doc = "0x08 - CSIC Pattern Control Register"]
    pub csic_ptn_ctrl_reg: crate::Reg<self::csic_top::csic_ptn_ctrl_reg::CSIC_PTN_CTRL_REG_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - CSIC Pattern Generation Length Register"]
    pub csic_ptn_len_reg: crate::Reg<self::csic_top::csic_ptn_len_reg::CSIC_PTN_LEN_REG_SPEC>,
    #[doc = "0x24 - CSIC Pattern Generation Address Register"]
    pub csic_ptn_addr_reg: crate::Reg<self::csic_top::csic_ptn_addr_reg::CSIC_PTN_ADDR_REG_SPEC>,
    #[doc = "0x28 - CSIC Pattern ISP Size Register"]
    pub csic_ptn_isp_size_reg:
        crate::Reg<self::csic_top::csic_ptn_isp_size_reg::CSIC_PTN_ISP_SIZE_REG_SPEC>,
    _reserved6: [u8; 0x74],
    #[doc = "0xa0 - CSIC DMA0 Input Select Register"]
    pub csic_dma0_input_sel_reg:
        crate::Reg<self::csic_top::csic_dma0_input_sel_reg::CSIC_DMA0_INPUT_SEL_REG_SPEC>,
    #[doc = "0xa4 - CSIC DMA1 Input Select Register"]
    pub csic_dma1_input_sel_reg:
        crate::Reg<self::csic_top::csic_dma1_input_sel_reg::CSIC_DMA1_INPUT_SEL_REG_SPEC>,
    _reserved8: [u8; 0x34],
    #[doc = "0xdc - CSIC BIST CS Register"]
    pub csic_bist_cs_reg: crate::Reg<self::csic_top::csic_bist_cs_reg::CSIC_BIST_CS_REG_SPEC>,
    #[doc = "0xe0 - CSIC BIST Control Register"]
    pub csic_bist_control_reg:
        crate::Reg<self::csic_top::csic_bist_control_reg::CSIC_BIST_CONTROL_REG_SPEC>,
    #[doc = "0xe4 - CSIC BIST Start Register"]
    pub csic_bist_start_reg:
        crate::Reg<self::csic_top::csic_bist_start_reg::CSIC_BIST_START_REG_SPEC>,
    #[doc = "0xe8 - CSIC BIST End Register"]
    pub csic_bist_end_reg: crate::Reg<self::csic_top::csic_bist_end_reg::CSIC_BIST_END_REG_SPEC>,
    #[doc = "0xec - CSIC BIST Data Mask Register"]
    pub csic_bist_data_mask_reg:
        crate::Reg<self::csic_top::csic_bist_data_mask_reg::CSIC_BIST_DATA_MASK_REG_SPEC>,
    #[doc = "0xf0 - CSIC MBUS REQ MAX Register"]
    pub csic_mbus_req_max_reg:
        crate::Reg<self::csic_top::csic_mbus_req_max_reg::CSIC_MBUS_REQ_MAX_REG_SPEC>,
    _reserved14: [u8; 0x0c],
    #[doc = "0x100 - CSIC Multi-Frame Mode Register"]
    pub csic_mulf_mod_reg: crate::Reg<self::csic_top::csic_mulf_mod_reg::CSIC_MULF_MOD_REG_SPEC>,
    #[doc = "0x104 - CSIC Multi-Frame Interrupt Register"]
    pub csic_mulf_int_reg: crate::Reg<self::csic_top::csic_mulf_int_reg::CSIC_MULF_INT_REG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CSIC_TOP"]
pub mod csic_top;
#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_PARSER0 { # [ doc = "0x00 - Parser Enable Register" ] pub prs_en_reg : crate :: Reg < self :: csic_parser0 :: prs_en_reg :: PRS_EN_REG_SPEC > , # [ doc = "0x04 - Parser NCSIC Interface Configuration Register" ] pub prs_ncsic_if_cfg_reg : crate :: Reg < self :: csic_parser0 :: prs_ncsic_if_cfg_reg :: PRS_NCSIC_IF_CFG_REG_SPEC > , _reserved2 : [ u8 ; 0x04 ] , # [ doc = "0x0c - Parser Capture Register" ] pub prs_cap_reg : crate :: Reg < self :: csic_parser0 :: prs_cap_reg :: PRS_CAP_REG_SPEC > , # [ doc = "0x10 - CSIC Parser Signal Status Register" ] pub csic_prs_signal_sta_reg : crate :: Reg < self :: csic_parser0 :: csic_prs_signal_sta_reg :: CSIC_PRS_SIGNAL_STA_REG_SPEC > , # [ doc = "0x14 - CSIC Parser NCSIC BT656 Header Configuration Register" ] pub csic_prs_ncsic_bt656_head_cfg_reg : crate :: Reg < self :: csic_parser0 :: csic_prs_ncsic_bt656_head_cfg_reg :: CSIC_PRS_NCSIC_BT656_HEAD_CFG_REG_SPEC > , _reserved5 : [ u8 ; 0x0c ] , # [ doc = "0x24 - Parser Channel_0 Input Format Register" ] pub prs_c0_infmt_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_infmt_reg :: PRS_C0_INFMT_REG_SPEC > , # [ doc = "0x28 - Parser Channel_0 Output Horizontal Size Register" ] pub prs_c0_output_hsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_output_hsize_reg :: PRS_C0_OUTPUT_HSIZE_REG_SPEC > , # [ doc = "0x2c - Parser Channel_0 Output Vertical Size Register" ] pub prs_c0_output_vsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_output_vsize_reg :: PRS_C0_OUTPUT_VSIZE_REG_SPEC > , # [ doc = "0x30 - Parser Channel_0 Input Parameter0 Register" ] pub prs_c0_input_para0_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_input_para0_reg :: PRS_C0_INPUT_PARA0_REG_SPEC > , # [ doc = "0x34 - Parser Channel_0 Input Parameter1 Register" ] pub prs_c0_input_para1_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_input_para1_reg :: PRS_C0_INPUT_PARA1_REG_SPEC > , # [ doc = "0x38 - Parser Channel_0 Input Parameter2 Register" ] pub prs_c0_input_para2_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_input_para2_reg :: PRS_C0_INPUT_PARA2_REG_SPEC > , # [ doc = "0x3c - Parser Channel_0 Input Parameter3 Register" ] pub prs_c0_input_para3_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_input_para3_reg :: PRS_C0_INPUT_PARA3_REG_SPEC > , # [ doc = "0x40 - Parser Channel_0 Interrupt Enable Register" ] pub prs_c0_int_en_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_int_en_reg :: PRS_C0_INT_EN_REG_SPEC > , # [ doc = "0x44 - Parser Channel_0 Interrupt Status Register" ] pub prs_c0_int_sta_reg : crate :: Reg < self :: csic_parser0 :: prs_c0_int_sta_reg :: PRS_C0_INT_STA_REG_SPEC > , # [ doc = "0x48 - Parser Channel_0 Line Time Register" ] pub prs_ch0_line_time_reg : crate :: Reg < self :: csic_parser0 :: prs_ch0_line_time_reg :: PRS_CH0_LINE_TIME_REG_SPEC > , _reserved15 : [ u8 ; 0xd8 ] , # [ doc = "0x124 - Parser Channel_1 Input Format Register" ] pub prs_c1_infmt_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_infmt_reg :: PRS_C1_INFMT_REG_SPEC > , # [ doc = "0x128 - Parser Channel_1 Output Horizontal Size" ] pub prs_c1_output_hsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_output_hsize_reg :: PRS_C1_OUTPUT_HSIZE_REG_SPEC > , # [ doc = "0x12c - Parser Channel_1 Output Vertical Size Register" ] pub prs_c1_output_vsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_output_vsize_reg :: PRS_C1_OUTPUT_VSIZE_REG_SPEC > , # [ doc = "0x130 - Parser Channel_1 Input Parameter0 Register" ] pub prs_c1_input_para0_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_input_para0_reg :: PRS_C1_INPUT_PARA0_REG_SPEC > , # [ doc = "0x134 - Parser Channel_1 Input Parameter1 Register" ] pub prs_c1_input_para1_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_input_para1_reg :: PRS_C1_INPUT_PARA1_REG_SPEC > , # [ doc = "0x138 - Parser Channel_1 Input Parameter2 Register" ] pub prs_c1_input_para2_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_input_para2_reg :: PRS_C1_INPUT_PARA2_REG_SPEC > , # [ doc = "0x13c - Parser Channel_1 Input Parameter3 Register" ] pub prs_c1_input_para3_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_input_para3_reg :: PRS_C1_INPUT_PARA3_REG_SPEC > , # [ doc = "0x140 - Parser Channel_1 Interrupt Enable Register" ] pub prs_c1_int_en_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_int_en_reg :: PRS_C1_INT_EN_REG_SPEC > , # [ doc = "0x144 - Parser Channel_1 Interrupt Status Register" ] pub prs_c1_int_sta_reg : crate :: Reg < self :: csic_parser0 :: prs_c1_int_sta_reg :: PRS_C1_INT_STA_REG_SPEC > , # [ doc = "0x148 - Parser Channel_1 Line Time Register" ] pub prs_ch1_line_time_reg : crate :: Reg < self :: csic_parser0 :: prs_ch1_line_time_reg :: PRS_CH1_LINE_TIME_REG_SPEC > , _reserved25 : [ u8 ; 0xd8 ] , # [ doc = "0x224 - Parser Channel_2 Input Format Register" ] pub prs_c2_infmt_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_infmt_reg :: PRS_C2_INFMT_REG_SPEC > , # [ doc = "0x228 - Parser Channel_2 Output Horizontal Size Register" ] pub prs_c2_output_hsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_output_hsize_reg :: PRS_C2_OUTPUT_HSIZE_REG_SPEC > , # [ doc = "0x22c - Parser Channel_2 Output Vertical Size Register" ] pub prs_c2_output_vsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_output_vsize_reg :: PRS_C2_OUTPUT_VSIZE_REG_SPEC > , # [ doc = "0x230 - Parser Channel_2 Input Parameter0 Register" ] pub prs_c2_input_para0_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_input_para0_reg :: PRS_C2_INPUT_PARA0_REG_SPEC > , # [ doc = "0x234 - Parser Channel_2 Input Parameter1 Register" ] pub prs_c2_input_para1_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_input_para1_reg :: PRS_C2_INPUT_PARA1_REG_SPEC > , # [ doc = "0x238 - Parser Channel_2 Input Parameter2 Register" ] pub prs_c2_input_para2_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_input_para2_reg :: PRS_C2_INPUT_PARA2_REG_SPEC > , # [ doc = "0x23c - Parser Channel_2 Input Parameter3 Register" ] pub prs_c2_input_para3_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_input_para3_reg :: PRS_C2_INPUT_PARA3_REG_SPEC > , # [ doc = "0x240 - Parser Channel_2 Interrupt Enable Register" ] pub prs_c2_int_en_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_int_en_reg :: PRS_C2_INT_EN_REG_SPEC > , # [ doc = "0x244 - Parser Channel_2 Interrupt Status Register" ] pub prs_c2_int_sta_reg : crate :: Reg < self :: csic_parser0 :: prs_c2_int_sta_reg :: PRS_C2_INT_STA_REG_SPEC > , # [ doc = "0x248 - Parser Channel_2 Line Time Register" ] pub prs_ch2_line_time_reg : crate :: Reg < self :: csic_parser0 :: prs_ch2_line_time_reg :: PRS_CH2_LINE_TIME_REG_SPEC > , _reserved35 : [ u8 ; 0xd8 ] , # [ doc = "0x324 - Parser Channel_3 Input Format Register" ] pub prs_c3_infmt_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_infmt_reg :: PRS_C3_INFMT_REG_SPEC > , # [ doc = "0x328 - Parser Channel_3 Output Horizontal Size Register" ] pub prs_c3_output_hsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_output_hsize_reg :: PRS_C3_OUTPUT_HSIZE_REG_SPEC > , # [ doc = "0x32c - Parser Channel_3 Output Vertical Size Register" ] pub prs_c3_output_vsize_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_output_vsize_reg :: PRS_C3_OUTPUT_VSIZE_REG_SPEC > , # [ doc = "0x330 - Parser Channel_3 Input Parameter0 Register" ] pub prs_c3_input_para0_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_input_para0_reg :: PRS_C3_INPUT_PARA0_REG_SPEC > , # [ doc = "0x334 - Parser Channel_3 Input Parameter1 Register" ] pub prs_c3_input_para1_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_input_para1_reg :: PRS_C3_INPUT_PARA1_REG_SPEC > , # [ doc = "0x338 - Parser Channel_3 Input Parameter2 Register" ] pub prs_c3_input_para2_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_input_para2_reg :: PRS_C3_INPUT_PARA2_REG_SPEC > , # [ doc = "0x33c - Parser Channel_3 Input Parameter3 Register" ] pub prs_c3_input_para3_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_input_para3_reg :: PRS_C3_INPUT_PARA3_REG_SPEC > , # [ doc = "0x340 - Parser Channel_3 Interrupt Enable Register" ] pub prs_c3_int_en_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_int_en_reg :: PRS_C3_INT_EN_REG_SPEC > , # [ doc = "0x344 - Parser Channel_3 Interrupt Status Register" ] pub prs_c3_int_sta_reg : crate :: Reg < self :: csic_parser0 :: prs_c3_int_sta_reg :: PRS_C3_INT_STA_REG_SPEC > , # [ doc = "0x348 - Parser Channel_3 Line Time Register" ] pub prs_ch3_line_time_reg : crate :: Reg < self :: csic_parser0 :: prs_ch3_line_time_reg :: PRS_CH3_LINE_TIME_REG_SPEC > , _reserved45 : [ u8 ; 0x01b4 ] , # [ doc = "0x500 - CSIC Parser NCSIC RX Signal0 Delay Adjust Register" ] pub csic_prs_ncsic_rx_signal0_dly_adj_reg : crate :: Reg < self :: csic_parser0 :: csic_prs_ncsic_rx_signal0_dly_adj_reg :: CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_REG_SPEC > , _reserved46 : [ u8 ; 0x10 ] , # [ doc = "0x514 - CSIC Parser NCSIC RX Signal5 Delay Adjust Register" ] pub csic_prs_ncsic_rx_signal5_dly_adj_reg : crate :: Reg < self :: csic_parser0 :: csic_prs_ncsic_rx_signal5_dly_adj_reg :: CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_REG_SPEC > , # [ doc = "0x518 - CSIC Parser NCSIC RX Signal6 Delay Adjust Register" ] pub csic_prs_ncsic_rx_signal6_dly_adj_reg : crate :: Reg < self :: csic_parser0 :: csic_prs_ncsic_rx_signal6_dly_adj_reg :: CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_REG_SPEC > , }
#[doc = r"Register block"]
#[doc = "CSIC_PARSER0"]
pub mod csic_parser0;
#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_DMA {
    #[doc = "0x00 - CSIC DMA Enable Register"]
    pub csic_dma_en_reg: crate::Reg<self::csic_dma::csic_dma_en_reg::CSIC_DMA_EN_REG_SPEC>,
    #[doc = "0x04 - CSIC DMA Configuration Register"]
    pub csic_dma_cfg_reg: crate::Reg<self::csic_dma::csic_dma_cfg_reg::CSIC_DMA_CFG_REG_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - CSIC DMA Horizontal Size Register"]
    pub csic_dma_hsize_reg: crate::Reg<self::csic_dma::csic_dma_hsize_reg::CSIC_DMA_HSIZE_REG_SPEC>,
    #[doc = "0x14 - CSIC DMA Vertical Size Register"]
    pub csic_dma_vsize_reg: crate::Reg<self::csic_dma::csic_dma_vsize_reg::CSIC_DMA_VSIZE_REG_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - CSIC DMA FIFO 0 Output Buffer-A Address Register"]
    pub csic_dma_f0_bufa_reg:
        crate::Reg<self::csic_dma::csic_dma_f0_bufa_reg::CSIC_DMA_F0_BUFA_REG_SPEC>,
    #[doc = "0x24 - CSIC DMA FIFO 0 Output Buffer-A Address Result Register"]
    pub csic_dma_f0_bufa_result_reg:
        crate::Reg<self::csic_dma::csic_dma_f0_bufa_result_reg::CSIC_DMA_F0_BUFA_RESULT_REG_SPEC>,
    #[doc = "0x28 - CSIC DMA FIFO 1 Output Buffer-A Address Register"]
    pub csic_dma_f1_bufa_reg:
        crate::Reg<self::csic_dma::csic_dma_f1_bufa_reg::CSIC_DMA_F1_BUFA_REG_SPEC>,
    #[doc = "0x2c - CSIC DMA FIFO 1 Output Buffer-A Address Result Register"]
    pub csic_dma_f1_bufa_result_reg:
        crate::Reg<self::csic_dma::csic_dma_f1_bufa_result_reg::CSIC_DMA_F1_BUFA_RESULT_REG_SPEC>,
    #[doc = "0x30 - CSIC DMA FIFO 2 Output Buffer-A Address Register"]
    pub csic_dma_f2_bufa_reg:
        crate::Reg<self::csic_dma::csic_dma_f2_bufa_reg::CSIC_DMA_F2_BUFA_REG_SPEC>,
    #[doc = "0x34 - CSIC DMA FIFO 2 Output Buffer-A Address Result Register"]
    pub csic_dma_f2_bufa_result_reg:
        crate::Reg<self::csic_dma::csic_dma_f2_bufa_result_reg::CSIC_DMA_F2_BUFA_RESULT_REG_SPEC>,
    #[doc = "0x38 - CSIC DMA Buffer Length Register"]
    pub csic_dma_buf_len_reg:
        crate::Reg<self::csic_dma::csic_dma_buf_len_reg::CSIC_DMA_BUF_LEN_REG_SPEC>,
    #[doc = "0x3c - CSIC DMA Flip Size Register"]
    pub csic_dma_flip_size_reg:
        crate::Reg<self::csic_dma::csic_dma_flip_size_reg::CSIC_DMA_FLIP_SIZE_REG_SPEC>,
    #[doc = "0x40 - CSIC DMA Video Input Timeout Threshold0 Register"]
    pub csic_dma_vi_to_th0_reg:
        crate::Reg<self::csic_dma::csic_dma_vi_to_th0_reg::CSIC_DMA_VI_TO_TH0_REG_SPEC>,
    #[doc = "0x44 - CSIC DMA Video Input Timeout Threshold1 Register"]
    pub csic_dma_vi_to_th1_reg:
        crate::Reg<self::csic_dma::csic_dma_vi_to_th1_reg::CSIC_DMA_VI_TO_TH1_REG_SPEC>,
    #[doc = "0x48 - CSIC DMA Video Input Timeout Counter Value Register"]
    pub csic_dma_vi_to_cnt_val_reg:
        crate::Reg<self::csic_dma::csic_dma_vi_to_cnt_val_reg::CSIC_DMA_VI_TO_CNT_VAL_REG_SPEC>,
    #[doc = "0x4c - CSIC DMA Capture Status Register"]
    pub csic_dma_cap_sta_reg:
        crate::Reg<self::csic_dma::csic_dma_cap_sta_reg::CSIC_DMA_CAP_STA_REG_SPEC>,
    #[doc = "0x50 - CSIC DMA Interrupt Enable Register"]
    pub csic_dma_int_en_reg:
        crate::Reg<self::csic_dma::csic_dma_int_en_reg::CSIC_DMA_INT_EN_REG_SPEC>,
    #[doc = "0x54 - CSIC DMA Interrupt Status Register"]
    pub csic_dma_int_sta_reg:
        crate::Reg<self::csic_dma::csic_dma_int_sta_reg::CSIC_DMA_INT_STA_REG_SPEC>,
    #[doc = "0x58 - CSIC DMA LINE Counter Register"]
    pub csic_dma_line_cnt_reg:
        crate::Reg<self::csic_dma::csic_dma_line_cnt_reg::CSIC_DMA_LINE_CNT_REG_SPEC>,
    #[doc = "0x5c - CSIC DMA Frame Counter Register"]
    pub csic_dma_frm_cnt_reg:
        crate::Reg<self::csic_dma::csic_dma_frm_cnt_reg::CSIC_DMA_FRM_CNT_REG_SPEC>,
    #[doc = "0x60 - CSIC DMA Frame Clock Counter Register"]
    pub csic_dma_frm_clk_cnt_reg:
        crate::Reg<self::csic_dma::csic_dma_frm_clk_cnt_reg::CSIC_DMA_FRM_CLK_CNT_REG_SPEC>,
    #[doc = "0x64 - CSIC DMA Accumulated And Internal Clock Counter Register"]
    pub csic_dma_acc_itnl_clk_cnt_reg: crate::Reg<
        self::csic_dma::csic_dma_acc_itnl_clk_cnt_reg::CSIC_DMA_ACC_ITNL_CLK_CNT_REG_SPEC,
    >,
    #[doc = "0x68 - CSIC DMA FIFO Statistic Register"]
    pub csic_dma_fifo_stat_reg:
        crate::Reg<self::csic_dma::csic_dma_fifo_stat_reg::CSIC_DMA_FIFO_STAT_REG_SPEC>,
    #[doc = "0x6c - CSIC DMA FIFO Threshold Register"]
    pub csic_dma_fifo_thrs_reg:
        crate::Reg<self::csic_dma::csic_dma_fifo_thrs_reg::CSIC_DMA_FIFO_THRS_REG_SPEC>,
    #[doc = "0x70 - CSIC DMA PCLK Statistic Register"]
    pub csic_dma_pclk_stat_reg:
        crate::Reg<self::csic_dma::csic_dma_pclk_stat_reg::CSIC_DMA_PCLK_STAT_REG_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0x80 - CSIC DMA BUF Address FIFO0 Entry Register"]
    pub csic_dma_buf_addr_fifo0_entry_reg: crate::Reg<
        self::csic_dma::csic_dma_buf_addr_fifo0_entry_reg::CSIC_DMA_BUF_ADDR_FIFO0_ENTRY_REG_SPEC,
    >,
    #[doc = "0x84 - CSIC DMA BUF Address FIFO1 Entry Register"]
    pub csic_dma_buf_addr_fifo1_entry_reg: crate::Reg<
        self::csic_dma::csic_dma_buf_addr_fifo1_entry_reg::CSIC_DMA_BUF_ADDR_FIFO1_ENTRY_REG_SPEC,
    >,
    #[doc = "0x88 - CSIC DMA BUF Address FIFO2 Entry Register"]
    pub csic_dma_buf_addr_fifo2_entry_reg: crate::Reg<
        self::csic_dma::csic_dma_buf_addr_fifo2_entry_reg::CSIC_DMA_BUF_ADDR_FIFO2_ENTRY_REG_SPEC,
    >,
    #[doc = "0x8c - CSIC DMA BUF Threshold Register"]
    pub csic_dma_buf_th_reg:
        crate::Reg<self::csic_dma::csic_dma_buf_th_reg::CSIC_DMA_BUF_TH_REG_SPEC>,
    #[doc = "0x90 - CSIC DMA BUF Address FIFO Content Register"]
    pub csic_dma_buf_addr_fifo_con_reg: crate::Reg<
        self::csic_dma::csic_dma_buf_addr_fifo_con_reg::CSIC_DMA_BUF_ADDR_FIFO_CON_REG_SPEC,
    >,
    #[doc = "0x94 - CSIC DMA Stored Frame Counter Register"]
    pub csic_dma_stored_frm_cnt_reg:
        crate::Reg<self::csic_dma::csic_dma_stored_frm_cnt_reg::CSIC_DMA_STORED_FRM_CNT_REG_SPEC>,
    _reserved31: [u8; 0x015c],
    #[doc = "0x1f4 - CSIC DMA Feature List Register"]
    pub csic_feature_reg: crate::Reg<self::csic_dma::csic_feature_reg::CSIC_FEATURE_REG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CSIC_DMA"]
pub mod csic_dma;
