pub fn progedu_css() -> &'static str {
    r#"
		svg.railroad rect.railroad_canvas {
		stroke-width: 0px;
		fill: white;
		}

		svg.railroad path {
		stroke-width: 1.1px;
		stroke: #0b2370;
		fill: none;
		}

		svg.railroad .debug {
		stroke-width: 1.1px;
		stroke: red;
		}

		svg.railroad text {
		font: 14px monospace;
		text-anchor: middle;
		fill: #0b2370;
		}

		svg.railroad text.comment {
		font: 12px monospace;
		}

		svg.railroad rect {
		stroke-width: 1.1px;
		stroke: #0b2370;
		fill:hsl(-290, 70%, 90%);
		}

		svg.railroad g.terminal > rect {
		fill: #ffffde;

		}

		svg.railroad g.nonterminal > rect {
		fill: #e3faff;
		}

		svg.railroad g.my-end > path {
		fill: #0b2370;
		stroke-width: 0.5px;
	}
            "#
}