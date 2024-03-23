module top
(
    input     [ 6:0] cmdOp,
    input     [ 2:0] cmdF3,
    input     [ 6:0] cmdF7,
    input            aluZero,
    output           pcSrc, 
    output reg       regWrite, 
    output reg       aluSrc,
    output reg       wdSrc,
    output reg [2:0] op
);
    //reg          branch;
    //reg          condZero;
    //assign pcSrc = branch & (aluZero == condZero);

    always @ (*) begin
        branch      = 1'b0;
        condZero    = 1'b0;
        regWrite    = 1'b0;
        aluSrc      = 1'b0;
        wdSrc       = 1'b0;
        op  = 3'b000;

        casez( {cmdF7, cmdF3, cmdOp} )
            { 7'b0000000, 3'b000, 7'b0110011 } : begin regWrite = 1'b1; op = 3'b000;  end
            { 7'b0000000, 3'b110, 7'b0110011 } : begin regWrite = 1'b1; op = 3'b001;   end
            { 7'b0000000, 3'b101, 7'b0110011 } : begin regWrite = 1'b1; op = 3'b010;  end
            { 7'b0000000, 3'b011, 7'b0110011 } : begin regWrite = 1'b1; op = 3'b011; end
            { 7'b0100000, 3'b000, 7'b0110011 } : begin regWrite = 1'b1; op = 3'b100;  end

            { 7'b???????, 3'b000, 7'b0010011 } : begin regWrite = 1'b1; aluSrc = 1'b1; op = 3'b000; end
            { 7'b???????, 3'b???, 7'b0110111 } : begin regWrite = 1'b1; wdSrc  = 1'b1; end

            { 7'b???????, 3'b000, 7'b1100011 } : begin op = 3'b000; end
            { 7'b???????, 3'b001, 7'b1100011 } : begin op = 3'b000; end
        endcase
    end
endmodule