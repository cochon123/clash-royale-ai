#!/bin/sh
# One-shot 06:00 deadline: stop this run's services and restore resident models.
systemctl --user stop cr-v9-recovery.timer cr-v9-suite.service cr-v9-train.service cr-v9-scale15k.service cr-v9-advantage.service cr-v9-cache.service cr-v9-judge.service
systemctl --user unmask --runtime lettera-qwen-asr.service
systemctl --user start lettera-qwen-asr.service comfyui.service
