use uom::si::heat_transfer::watt_per_square_meter_kelvin;


/// Validation test using the experimental data,
///
/// for now, results look good! within 0.2 degC of expt data
/// so long as ambient htc is around 6 W/(m^2 K)
///
/// On page 46 and 47 of Zweibaum's thesis, the following transient was done 
/// power step transient at the following approximate times:
///
/// Note: these are approximate
/// Time(s), Heater power (W)
/// 3052.544,2528.239
/// 3282.08,2518.272
/// 3365.044,4232.558
/// 3984.513,4242.525
/// 4050.885,6076.412
/// 4664.823,6086.379
/// 4709.071,7093.023
/// 5372.788,7093.023
/// 5405.973,7611.296
/// 6030.973,7601.329
/// 6069.69,7162.791
/// 6871.681,7152.824
/// 6888.274,5438.538
/// 7529.867,5438.538
/// 7551.991,2707.641
/// 7975.111,2707.641
///
/// The resulting approx temperatures were:
///
/// Time (s),Heater Inlet Temp (degC),Heater Outlet Temp (degC),CTAH Inlet Temp (degC),CTAH Outlet Temp (degC)
/// 3050,78.852,86.976,86.173,80.213
/// 3060,78.852,86.984,86.173,80.281
/// 3070,78.852,86.991,86.173,80.206
/// 3080,78.852,86.999,86.173,80.131
/// 3090,78.852,87.006,86.173,80.056
/// 3100,78.852,87.014,86.173,79.981
/// 3110,78.852,87.021,86.173,79.906
/// 3120,78.852,87.029,86.173,79.831
/// 3130,78.852,87.036,86.173,79.755
/// 3140,78.852,87.042,86.173,79.68
/// 3150,78.852,87.048,86.173,79.714
/// 3160,78.852,87.053,86.173,79.819
/// 3170,78.852,87.059,86.173,79.924
/// 3180,78.852,87.064,86.178,80.03
/// 3190,78.852,87.069,86.195,80.113
/// 3200,78.852,87.075,86.212,80.095
/// 3210,78.852,87.08,86.229,80.076
/// 3220,78.852,87.086,86.247,80.058
/// 3230,78.852,87.091,86.264,80.039
/// 3240,78.852,87.097,86.281,80.021
/// 3250,78.852,87.102,86.298,80.002
/// 3260,78.852,87.092,86.31,79.984
/// 3270,78.852,87.068,86.318,79.968
/// 3280,78.857,87.044,86.326,79.952
/// 3290,78.873,87.02,86.334,79.936
/// 3300,78.889,86.996,86.342,79.92
/// 3310,78.905,86.972,86.35,79.904
/// 3320,78.921,87.33,86.358,79.888
/// 3330,78.937,87.69,86.366,79.872
/// 3340,78.953,88.051,86.391,79.856
/// 3350,78.969,88.411,86.471,79.951
/// 3360,78.985,88.786,86.551,80.101
/// 3370,79.001,90.709,86.631,80.251
/// 3380,79.017,91.022,86.711,80.402
/// 3390,79.033,91.16,87.162,80.617
/// 3400,79.049,91.297,88.633,81.038
/// 3410,79.065,91.434,89.114,81.445
/// 3420,79.081,91.572,89.595,81.345
/// 3430,79.097,91.709,90.017,81.245
/// 3440,79.113,91.847,90.337,81.145
/// 3450,79.217,91.978,90.658,80.65
/// 3460,79.357,92.076,90.971,80.361
/// 3470,79.497,92.173,91.102,80.325
/// 3480,79.564,92.271,91.233,80.289
/// 3490,79.511,92.369,91.364,80.253
/// 3500,79.459,92.466,91.495,80.217
/// 3510,79.406,92.564,91.626,80.183
/// 3520,79.354,92.662,91.757,80.183
/// 3530,79.301,92.759,91.764,80.183
/// 3540,79.248,92.823,91.764,80.183
/// 3550,79.196,92.805,91.764,80.183
/// 3560,79.143,92.786,91.764,80.182
/// 3570,79.118,92.768,91.764,80.175
/// 3580,79.118,92.749,91.764,80.167
/// 3590,79.118,92.731,91.764,80.16
/// 3600,79.118,92.712,91.764,80.152
/// 3610,79.118,92.694,91.764,80.145
/// 3620,79.118,92.675,91.764,80.137
/// 3630,79.118,92.657,91.756,80.13
/// 3640,79.118,92.638,91.739,80.122
/// 3650,79.118,92.62,91.722,80.118
/// 3660,79.118,92.601,91.705,80.126
/// 3670,79.118,92.583,91.688,80.133
/// 3680,79.118,92.564,91.67,80.141
/// 3690,79.111,92.593,91.653,80.148
/// 3700,79.104,92.627,91.636,80.156
/// 3710,79.096,92.662,91.619,80.163
/// 3720,79.089,92.696,91.602,80.171
/// 3730,79.081,92.73,91.585,80.178
/// 3740,79.074,92.765,91.567,80.174
/// 3750,79.066,92.799,91.581,80.148
/// 3760,79.059,92.829,91.602,80.122
/// 3770,79.051,92.829,91.624,80.097
/// 3780,79.04,92.829,91.645,80.071
/// 3790,79.029,92.829,91.666,80.045
/// 3800,79.018,92.829,91.687,80.019
/// 3810,79.007,92.829,91.708,79.994
/// 3820,78.997,92.829,91.73,79.974
/// 3830,78.986,92.829,91.751,79.958
/// 3840,78.985,92.829,91.769,79.942
/// 3850,78.985,92.829,91.781,79.926
/// 3860,78.985,92.829,91.794,79.91
/// 3870,78.985,92.829,91.807,79.894
/// 3880,78.985,92.829,91.819,79.878
/// 3890,78.985,92.793,91.832,79.861
/// 3900,78.985,92.756,91.845,79.86
/// 3910,78.985,92.719,91.857,79.892
/// 3920,78.985,92.682,91.87,79.924
/// 3930,78.985,92.645,91.883,79.956
/// 3940,78.985,92.608,91.895,79.988
/// 3950,78.985,92.571,91.897,80.02
/// 3960,78.985,92.599,91.897,80.052
/// 3970,78.985,92.647,91.897,80.084
/// 3980,78.985,92.695,91.897,80.116
/// 3990,78.985,92.743,91.897,80.108
/// 4000,78.985,92.791,91.897,80.099
/// 4010,78.998,92.949,91.909,80.091
/// 4020,79.016,93.483,91.962,80.082
/// 4030,79.033,94.017,92.016,80.074
/// 4040,79.05,94.551,92.069,80.065
/// 4050,79.067,95.086,92.123,80.056
/// 4060,79.084,95.57,92.395,80.131
/// 4070,79.101,95.899,93.389,80.468
/// 4080,79.12,96.227,94.573,80.804
/// 4090,79.175,96.556,95.114,81.11
/// 4100,79.231,96.884,95.655,81.38
/// 4110,79.286,97.213,95.994,81.514
/// 4120,79.342,97.541,96.286,81.257
/// 4130,79.397,97.87,96.578,80.999
/// 4140,79.452,98.199,96.87,80.742
/// 4150,79.508,98.255,96.957,80.529
/// 4160,79.517,98.293,97.031,80.389
/// 4170,79.517,98.331,97.104,80.248
/// 4180,79.517,98.369,97.178,80.116
/// 4190,79.517,98.407,97.251,80.11
/// 4200,79.517,98.445,97.325,80.103
/// 4210,79.517,98.483,97.398,80.097
/// 4220,79.507,98.521,97.471,80.091
/// 4230,79.452,98.559,97.545,80.084
/// 4240,79.396,98.597,97.618,80.078
/// 4250,79.341,98.622,97.64,80.072
/// 4260,79.285,98.628,97.66,80.065
/// 4270,79.23,98.635,97.68,80.059
/// 4280,79.174,98.642,97.7,80.053
/// 4290,79.119,98.648,97.72,80.05
/// 4300,79.118,98.655,97.74,80.05
/// 4310,79.118,98.662,97.76,80.05
/// 4320,79.118,98.668,97.78,80.05
/// 4330,79.118,98.675,97.8,80.05
/// 4340,79.118,98.682,97.82,80.05
/// 4350,79.118,98.69,97.796,80.05
/// 4360,79.118,98.701,97.772,80.05
/// 4370,79.115,98.711,97.748,80.05
/// 4380,79.103,98.722,97.724,80.05
/// 4390,79.09,98.732,97.7,80.05
/// 4400,79.077,98.742,97.676,80.05
/// 4410,79.065,98.753,97.652,80.05
/// 4420,79.052,98.763,97.628,80.05
/// 4430,79.039,98.774,97.615,80.05
/// 4440,79.027,98.784,97.606,80.05
/// 4450,79.014,98.795,97.597,80.05
/// 4460,79.001,98.805,97.589,80.05
/// 4470,78.989,98.816,97.58,80.05
/// 4480,78.992,98.819,97.572,80.053
/// 4490,79.001,98.819,97.563,80.07
/// 4500,79.01,98.819,97.555,80.087
/// 4510,79.019,98.819,97.564,80.104
/// 4520,79.029,98.819,97.575,80.116
/// 4530,79.038,98.819,97.586,80.116
/// 4540,79.047,98.819,97.597,80.116
/// 4550,79.057,98.819,97.608,80.116
/// 4560,79.068,98.819,97.619,80.116
/// 4570,79.079,98.819,97.621,80.116
/// 4580,79.09,98.819,97.621,80.108
/// 4590,79.101,98.807,97.621,80.097
/// 4600,79.112,98.788,97.621,80.086
/// 4610,79.115,98.77,97.621,80.076
/// 4620,79.107,98.751,97.621,80.065
/// 4630,79.1,98.733,97.621,80.054
/// 4640,79.092,98.714,97.621,80.076
/// 4650,79.085,98.696,97.621,80.116
/// 4660,79.077,98.797,97.621,80.156
/// 4670,79.07,99.051,97.634,80.196
/// 4680,79.062,99.305,97.674,80.236
/// 4690,79.055,99.559,97.714,80.277
/// 4700,79.052,99.812,97.756,80.317
/// 4710,79.052,100.011,97.936,80.397
/// 4720,79.052,100.143,98.116,80.477
/// 4730,79.052,100.275,98.695,80.557
/// 4740,79.052,100.407,99.307,80.638
/// 4750,79.052,100.539,99.47,80.716
/// 4760,79.052,100.672,99.633,80.731
/// 4770,79.086,101.014,99.796,80.746
/// 4780,79.179,101.287,99.959,80.761
/// 4790,79.273,101.311,100.122,80.776
/// 4800,79.366,101.336,100.285,80.719
/// 4810,79.46,101.36,100.448,80.614
/// 4820,79.505,101.384,100.547,80.509
/// 4830,79.472,101.408,100.541,80.404
/// 4840,79.439,101.432,100.535,80.312
/// 4850,79.407,101.456,100.529,80.285
/// 4860,79.374,101.48,100.523,80.258
/// 4870,79.341,101.504,100.517,80.232
/// 4880,79.316,101.528,100.511,80.205
/// 4890,79.31,101.552,100.505,80.178
/// 4900,79.303,101.576,100.499,80.151
/// 4910,79.297,101.6,100.493,80.125
/// 4920,79.291,101.608,100.487,80.098
/// 4930,79.284,101.594,100.485,80.071
/// 4940,79.278,101.579,100.491,80.046
/// 4950,79.272,101.565,100.498,80.024
/// 4960,79.265,101.55,100.505,80.003
/// 4970,79.259,101.536,100.511,79.982
/// 4980,79.253,101.521,100.518,79.961
/// 4990,79.238,101.507,100.525,79.94
/// 5000,79.221,101.493,100.531,79.918
/// 5010,79.204,101.478,100.538,79.897
/// 5020,79.186,101.464,100.545,79.876
/// 5030,79.169,101.449,100.549,79.855
/// 5040,79.152,101.435,100.549,79.85
/// 5050,79.135,101.42,100.549,79.85
/// 5060,79.118,101.418,100.549,79.85
/// 5070,79.13,101.424,100.549,79.85
/// 5080,79.142,101.43,100.549,79.85
/// 5090,79.154,101.436,100.549,79.85
/// 5100,79.166,101.442,100.549,79.85
/// 5110,79.178,101.448,100.549,79.85
/// 5120,79.179,101.454,100.549,79.856
/// 5130,79.167,101.46,100.529,79.866
/// 5140,79.155,101.466,100.508,79.877
/// 5150,79.143,101.472,100.487,79.888
/// 5160,79.131,101.478,100.466,79.899
/// 5170,79.119,101.489,100.445,79.91
/// 5180,79.107,101.507,100.423,79.925
/// 5190,79.095,101.524,100.402,79.945
/// 5200,79.083,101.541,100.381,79.965
/// 5210,79.071,101.558,100.36,79.985
/// 5220,79.059,101.575,100.364,80.005
/// 5230,79.052,101.592,100.392,80.025
/// 5240,79.052,101.61,100.421,80.045
/// 5250,79.052,101.614,100.449,80.06
/// 5260,79.052,101.614,100.477,80.073
/// 5270,79.052,101.614,100.505,80.087
/// 5280,79.052,101.614,100.534,80.1
/// 5290,79.052,101.614,100.562,80.113
/// 5300,79.052,101.614,100.59,80.127
/// 5310,79.052,101.614,100.618,80.14
/// 5320,79.052,101.614,100.646,80.153
/// 5330,79.052,101.614,100.674,80.167
/// 5340,79.052,101.614,100.702,80.18
/// 5350,79.052,101.614,100.729,80.154
/// 5360,79.052,101.674,100.757,80.116
/// 5370,79.052,101.762,100.785,80.079
/// 5380,79.052,101.851,100.813,80.041
/// 5390,79.052,101.939,100.831,80.003
/// 5400,79.052,102.028,100.848,79.966
/// 5410,79.052,102.116,100.865,79.928
/// 5420,79.053,102.205,100.885,79.891
/// 5430,79.085,102.294,101.006,79.853
/// 5440,79.118,102.382,101.126,79.975
/// 5450,79.151,102.471,101.246,80.11
/// 5460,79.184,102.56,101.366,80.245
/// 5470,79.216,102.65,101.486,80.381
/// 5480,79.249,102.74,101.607,80.437
/// 5490,79.267,102.83,101.727,80.411
/// 5500,79.285,102.92,101.847,80.385
/// 5510,79.302,103.01,101.951,80.359
/// 5520,79.319,103.1,101.975,80.334
/// 5530,79.336,103.148,101.999,80.308
/// 5540,79.353,103.155,102.023,80.282
/// 5550,79.37,103.162,102.047,80.256
/// 5560,79.381,103.168,102.071,80.239
/// 5570,79.364,103.175,102.086,80.225
/// 5580,79.347,103.182,102.096,80.211
/// 5590,79.33,103.188,102.106,80.197
/// 5600,79.312,103.195,102.116,80.183
/// 5610,79.295,103.202,102.126,80.168
/// 5620,79.278,103.208,102.136,80.154
/// 5630,79.261,103.209,102.146,80.14
/// 5640,79.243,103.204,102.146,80.126
/// 5650,79.225,103.199,102.146,80.116
/// 5660,79.207,103.194,102.146,80.116
/// 5670,79.189,103.189,102.146,80.116
/// 5680,79.171,103.185,102.146,80.116
/// 5690,79.153,103.18,102.146,80.116
/// 5700,79.135,103.175,102.146,80.116
/// 5710,79.117,103.17,102.146,80.116
/// 5720,79.099,103.165,102.146,80.116
/// 5730,79.081,103.161,102.146,80.116
/// 5740,79.063,103.156,102.146,80.114
/// 5750,79.052,103.151,102.156,80.108
/// 5760,79.052,103.146,102.181,80.102
/// 5770,79.052,103.145,102.207,80.096
/// 5780,79.052,103.145,102.233,80.09
/// 5790,79.052,103.145,102.259,80.084
/// 5800,79.052,103.145,102.284,80.078
/// 5810,79.052,103.145,102.31,80.072
/// 5820,79.052,103.145,102.336,80.066
/// 5830,79.052,103.145,102.331,80.06
/// 5840,79.052,103.145,102.307,80.054
/// 5850,79.052,103.145,102.283,80.046
/// 5860,79.052,103.145,102.259,80.037
/// 5870,79.052,103.145,102.235,80.028
/// 5880,79.052,103.145,102.211,80.019
/// 5890,79.052,103.124,102.187,80.009
/// 5900,79.052,103.084,102.163,80
/// 5910,79.052,103.044,102.146,79.991
/// 5920,79.052,103.004,102.146,79.985
/// 5930,79.052,102.964,102.146,79.994
/// 5940,79.052,102.924,102.146,80.004
/// 5950,79.052,102.884,102.146,80.013
/// 5960,79.052,102.907,102.146,80.022
/// 5970,79.052,102.939,102.154,80.031
/// 5980,79.052,102.971,102.188,80.041
/// 5990,79.052,103.003,102.222,80.05
/// 6000,79.052,103.035,102.257,80.078
/// 6010,79.052,103.067,102.266,80.105
/// 6020,79.052,103.099,102.226,80.133
/// 6030,79.052,103.131,102.186,80.161
/// 6040,79.052,103.058,102.144,80.189
/// 6050,79.052,102.905,101.955,80.216
/// 6060,79.04,102.752,101.766,80.244
/// 6070,79.006,102.599,101.577,80.058
/// 6080,78.973,102.446,101.368,79.817
/// 6090,78.94,102.293,101.032,79.577
/// 6100,78.906,102.191,100.695,79.336
/// 6110,78.873,102.145,100.508,79.54
/// 6120,78.839,102.1,100.577,79.78
/// 6130,78.806,102.054,100.645,80.02
/// 6140,78.773,102.008,100.714,80.096
/// 6150,78.739,101.962,100.743,80.062
/// 6160,78.729,101.916,100.732,80.027
/// 6170,78.754,101.871,100.722,79.993
/// 6180,78.78,101.825,100.711,79.958
/// 6190,78.806,101.779,100.7,79.924
/// 6200,78.832,101.733,100.689,79.89
/// 6210,78.858,101.687,100.682,79.855
/// 6220,78.883,101.686,100.682,79.905
/// 6230,78.909,101.692,100.682,79.97
/// 6240,78.926,101.699,100.682,80.035
/// 6250,78.939,101.705,100.682,80.099
/// 6260,78.952,101.711,100.682,80.164
/// 6270,78.964,101.718,100.682,80.229
/// 6280,78.977,101.724,100.682,80.293
/// 6290,78.99,101.73,100.682,80.301
/// 6300,79.002,101.737,100.682,80.276
/// 6310,79.015,101.743,100.682,80.252
/// 6320,79.028,101.749,100.682,80.228
/// 6330,79.04,101.756,100.682,80.204
/// 6340,79.053,101.764,100.682,80.18
/// 6350,79.068,101.771,100.682,80.156
/// 6360,79.083,101.778,100.682,80.132
/// 6370,79.098,101.785,100.698,80.114
/// 6380,79.113,101.792,100.716,80.107
/// 6390,79.128,101.799,100.734,80.101
/// 6400,79.143,101.806,100.752,80.094
/// 6410,79.158,101.813,100.771,80.087
/// 6420,79.173,101.814,100.789,80.081
/// 6430,79.189,101.814,100.807,80.074
/// 6440,79.204,101.814,100.825,80.067
/// 6450,79.219,101.814,100.843,80.061
/// 6460,79.234,101.814,100.861,80.054
/// 6470,79.249,101.814,100.879,80.05
/// 6480,79.237,101.814,100.866,80.05
/// 6490,79.22,101.814,100.847,80.05
/// 6500,79.203,101.814,100.828,80.05
/// 6510,79.186,101.814,100.809,80.05
/// 6520,79.168,101.814,100.79,80.05
/// 6530,79.151,101.814,100.771,80.05
/// 6540,79.134,101.814,100.752,80.048
/// 6550,79.117,101.814,100.733,80.04
/// 6560,79.1,101.814,100.714,80.031
/// 6570,79.083,101.814,100.695,80.023
/// 6580,79.065,101.814,100.688,80.014
/// 6590,79.053,101.824,100.708,80.005
/// 6600,79.059,101.838,100.728,79.997
/// 6610,79.065,101.853,100.748,79.988
/// 6620,79.072,101.867,100.768,79.98
/// 6630,79.078,101.881,100.788,79.972
/// 6640,79.084,101.895,100.808,79.964
/// 6650,79.091,101.909,100.829,79.956
/// 6660,79.097,101.923,100.849,79.948
/// 6670,79.103,101.937,100.869,79.94
/// 6680,79.11,101.945,100.878,79.932
/// 6690,79.116,101.939,100.868,79.924
/// 6700,79.124,101.933,100.858,79.918
/// 6710,79.132,101.927,100.848,79.93
/// 6720,79.14,101.921,100.838,79.942
/// 6730,79.148,101.915,100.828,79.954
/// 6740,79.156,101.909,100.818,79.966
/// 6750,79.164,101.903,100.815,79.978
/// 6760,79.172,101.897,100.815,79.997
/// 6770,79.18,101.891,100.815,80.021
/// 6780,79.179,101.885,100.815,80.045
/// 6790,79.163,101.873,100.815,80.07
/// 6800,79.147,101.846,100.815,80.094
/// 6810,79.131,101.82,100.815,80.116
/// 6820,79.115,101.793,100.824,80.116
/// 6830,79.099,101.766,100.843,80.116
/// 6840,79.083,101.605,100.861,80.116
/// 6850,79.067,101.094,100.88,80.116
/// 6860,79.052,100.583,100.898,80.116
/// 6870,79.061,100.072,100.917,80.116
/// 6880,79.071,99.561,100.935,80.116
/// 6890,79.08,99.271,100.799,79.98
/// 6900,79.089,99.021,100.294,79.82
/// 6910,79.098,98.771,99.789,79.66
/// 6920,79.108,98.52,99.086,79.5
/// 6930,79.117,98.27,98.205,79.339
/// 6940,79.074,98.019,97.306,78.641
/// 6950,79.022,97.797,96.917,78.293
/// 6960,78.97,97.682,96.692,78.083
/// 6970,78.919,97.568,96.466,78.059
/// 6980,78.657,97.453,96.241,78.191
/// 6990,78.392,97.339,95.932,78.323
/// 7000,78.169,97.224,95.451,78.455
/// 7010,78.088,97.11,95.013,78.587
/// 7020,78.008,96.996,94.908,78.72
/// 7030,77.928,96.881,94.803,78.907
/// 7040,77.848,96.767,94.697,79.094
/// 7050,77.808,96.652,94.592,79.281
/// 7060,77.895,96.538,94.559,79.468
/// 7070,77.983,96.477,94.559,79.654
/// 7080,78.07,96.455,94.559,79.809
/// 7090,78.158,96.433,94.559,79.963
/// 7100,78.245,96.411,94.559,80.118
/// 7110,78.327,96.389,94.591,80.249
/// 7120,78.375,96.367,94.638,80.243
/// 7130,78.423,96.345,94.684,80.237
/// 7140,78.471,96.324,94.73,80.231
/// 7150,78.519,96.302,94.776,80.225
/// 7160,78.567,96.28,94.823,80.219
/// 7170,78.613,96.258,94.869,80.213
/// 7180,78.658,96.236,94.94,80.207
/// 7190,78.703,96.238,95.036,80.201
/// 7200,78.748,96.276,95.133,80.195
/// 7210,78.793,96.315,95.172,80.189
/// 7220,78.839,96.353,95.19,80.183
/// 7230,78.884,96.391,95.209,80.183
/// 7240,78.929,96.429,95.227,80.183
/// 7250,78.974,96.468,95.246,80.183
/// 7260,78.996,96.506,95.264,80.183
/// 7270,79.01,96.544,95.283,80.183
/// 7280,79.024,96.582,95.291,80.183
/// 7290,79.038,96.621,95.291,80.183
/// 7300,79.052,96.659,95.291,80.183
/// 7310,79.066,96.687,95.291,80.183
/// 7320,79.08,96.679,95.291,80.183
/// 7330,79.095,96.671,95.291,80.183
/// 7340,79.109,96.662,95.283,80.183
/// 7350,79.116,96.654,95.264,80.183
/// 7360,79.11,96.646,95.246,80.183
/// 7370,79.104,96.637,95.227,80.183
/// 7380,79.098,96.629,95.209,80.183
/// 7390,79.092,96.621,95.19,80.183
/// 7400,79.086,96.612,95.172,80.183
/// 7410,79.08,96.604,95.158,80.183
/// 7420,79.074,96.596,95.158,80.183
/// 7430,79.068,96.588,95.158,80.183
/// 7440,79.062,96.579,95.158,80.183
/// 7450,79.056,96.571,95.158,80.183
/// 7460,79.047,96.563,95.158,80.183
/// 7470,79.031,96.539,95.158,80.183
/// 7480,79.015,96.435,95.158,80.182
/// 7490,78.999,96.331,95.163,80.164
/// 7500,78.983,96.227,95.174,80.145
/// 7510,78.967,96.123,95.185,80.127
/// 7520,78.951,96.019,95.196,80.108
/// 7530,78.935,95.914,95.207,80.09
/// 7540,78.919,95.81,95.218,80.071
/// 7550,78.935,95.706,94.674,80.053
/// 7560,78.952,94.075,93.312,79.709
/// 7570,78.969,92.176,92.337,79.308
/// 7580,78.978,90.411,91.495,78.908
/// 7590,78.898,90.148,90.654,78.528
/// 7600,78.818,89.885,90.211,78.168
/// 7610,78.738,89.622,89.778,77.807
/// 7620,78.658,89.359,89.199,77.409
/// 7630,78.558,89.096,88.137,76.965
/// 7640,78.278,88.833,87.8,77.4
/// 7650,77.997,88.571,87.463,77.836
/// 7660,77.717,88.308,87.159,78.272
/// 7670,77.529,88.069,86.918,78.708
/// 7680,77.369,87.914,86.678,78.955
/// 7690,77.209,87.76,86.438,79.11
/// 7700,77.176,87.605,86.329,79.264
/// 7710,77.296,87.451,86.22,79.419
/// 7720,77.416,87.296,86.111,79.618
/// 7730,77.537,87.142,86.001,79.828
/// 7740,77.658,86.987,85.892,80.038
/// 7750,77.83,86.954,85.783,80.249
/// 7760,78.002,87.059,85.852,80.375
/// 7770,78.173,87.163,85.938,80.355
/// 7780,78.342,87.268,86.024,80.335
/// 7790,78.497,87.372,86.11,80.315
/// 7800,78.651,87.477,86.196,80.295
/// 7810,78.806,87.581,86.282,80.275
/// 7820,78.929,87.686,86.368,80.255
/// 7830,78.966,87.791,86.451,80.256
/// 7840,79.003,87.895,86.521,80.264
/// 7850,79.039,88,86.591,80.273
/// 7860,79.076,88.104,86.661,80.282
/// 7870,79.113,88.209,86.731,80.29
/// 7880,79.15,88.254,86.801,80.299
/// 7890,79.182,88.278,86.872,80.307
/// 7900,79.142,88.302,86.914,80.316
/// 7910,79.102,88.326,86.931,80.271
/// 7920,79.062,88.35,86.948,80.225
/// 7930,79.021,88.374,86.966,80.179
#[test]
pub fn steady_state_test_for_heater_v1_eight_nodes_validation(){
    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::prelude::beta_testing::*;

    use core::time;
    use std::{thread::{self}, time::SystemTime};
    use uom::{si::{time::second, power::kilowatt}, ConstZero};
    use uom::si::mass_rate::kilogram_per_second;
    // construct structs



    // heater v1 example
    let heater_power = Power::new::<kilowatt>(2.53);
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(78.852);
    let final_experimental_outlet_temp =
        ThermodynamicTemperature::new::<degree_celsius>(86.976);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_inner_temperature_nodes: usize = 10-2;
    
    let mut heater_v1 = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_with_annular_pipe(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut heater_top_head
        = InsulatedPorousMediaFluidComponent::new_ciet_v1_top_head(
            initial_temperature, 
            ambient_air_temp, 
            0);

    let mut heater_bottom_head 
        = InsulatedPorousMediaFluidComponent::new_ciet_v1_bottom_head(
            initial_temperature, 
            ambient_air_temp, 
            0);
    // note: mx10 potentially has a memory leak
    let mut static_mixer_mx_10_object: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_2_mx10(
        initial_temperature,
        ambient_air_temp);

    let mut static_mixer_mx_10_pipe: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_pipe_2a_mx10(
        initial_temperature,
        ambient_air_temp);

    // heat transfer coeff calibrated to 6.0 W/(m^2 K) 
    let htc_calibrated = HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

    heater_v1.heat_transfer_to_ambient = htc_calibrated;
    heater_top_head.heat_transfer_to_ambient = htc_calibrated;
    heater_bottom_head.heat_transfer_to_ambient = htc_calibrated;
    static_mixer_mx_10_object.heat_transfer_to_ambient = htc_calibrated;
    static_mixer_mx_10_pipe.heat_transfer_to_ambient = htc_calibrated;

    //let struct_support_equiv_diameter: Length = Length::new::<inch>(0.5);
    //let struc_support_equiv_length: Length = Length::new::<foot>(1.0);

    //let mut structural_support_heater_bottom_head: HeatTransferEntity 
    //= SingleCVNode::new_cylinder(
    //    struc_support_equiv_length,
    //    struct_support_equiv_diameter,
    //    SolidMaterial::SteelSS304L.into(),
    //    initial_temperature,
    //    Pressure::new::<atmosphere>(1.0),
    //).unwrap();

    //let mut structural_support_heater_top_head: HeatTransferEntity = 
    //structural_support_heater_bottom_head.clone();

    //let approx_support_conductance: ThermalConductance = {

    //    // for conductance, it is the half length that counts 
    //    //
    //    // bc -------- (support cv) ------------- heater head

    //    let conductivity = SolidMaterial::SteelSS304L.try_get_thermal_conductivity(
    //        initial_temperature
    //    ).unwrap();

    //    let xs_area_support = PI * 0.25 * struct_support_equiv_diameter 
    //    * struct_support_equiv_diameter;
    //    

    //    0.5 * conductivity * xs_area_support / struc_support_equiv_length

    //};

    //let support_conductance_interaction = HeatTransferInteractionType::
    //    UserSpecifiedThermalConductance(approx_support_conductance);


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    //let mut ambient_air_temp_bc: HeatTransferEntity = 
    //inlet_bc.clone();

    // time settings 

    let max_time = Time::new::<second>(300.0);
    let timestep = Time::new::<second>(0.05);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut final_outlet_temp = ThermodynamicTemperature::ZERO;

    let loop_time = SystemTime::now();
    // main loop
    // note: possible memory leak
    
    let main_loop = thread::spawn( move || {
        while max_time > simulation_time {

            // time start 
            let loop_time_start = loop_time.elapsed().unwrap();
            // create interactions 


            // let's get heater temperatures for post processing
            // as well as the interaction
            // for simplicity, i use the boussineseq approximation,
            // which assumes that heat transfer is governed by 
            // average density (which doesn't change much for liquid 
            // anyway)

            let connect_static_mixer_10 = true; 

            let mut therminol_array_clone: FluidArray 
            = heater_v1.pipe_fluid_array.clone().try_into().unwrap();

            let _therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

            let heater_surface_array_clone: SolidColumn 
            = heater_v1.pipe_shell.clone().try_into().unwrap();

            let heater_surface_array_temp: Vec<ThermodynamicTemperature> = 
            heater_surface_array_clone.get_temperature_vector().unwrap();

            let heater_fluid_bulk_temp: ThermodynamicTemperature = 
            therminol_array_clone.try_get_bulk_temperature().unwrap();

            let heater_top_head_bare_therminol_clone: FluidArray = 
            heater_top_head.pipe_fluid_array.clone().try_into().unwrap();

            let heater_top_head_exit_temperature: ThermodynamicTemperature = 
            heater_top_head_bare_therminol_clone.get_temperature_vector()
                .unwrap().into_iter().last().unwrap();

            if connect_static_mixer_10 {
                let static_mixer_therminol_clone: FluidArray = 
                static_mixer_mx_10_object.pipe_fluid_array.clone().try_into().unwrap();

                let _static_mixer_exit_temperature: ThermodynamicTemperature
                = static_mixer_therminol_clone.get_temperature_vector().unwrap()
                    .into_iter().last().unwrap();

                let static_mixer_pipe_therminol_clone: FluidArray = 
                static_mixer_mx_10_pipe.pipe_fluid_array.clone().try_into().unwrap();

                let bt_12_temperature: ThermodynamicTemperature = 
                static_mixer_pipe_therminol_clone.get_temperature_vector().unwrap() 
                    .into_iter().last().unwrap();

                // bt_12_temperature, which is actually the output temperature of static 
                // mixer 10
                dbg!(bt_12_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));
            }

            let heater_therminol_avg_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heater_fluid_bulk_temp).unwrap();

            let generic_advection_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                heater_therminol_avg_density,
                heater_therminol_avg_density,
            );
            // all unused values to try and mitigate memory leaking
            {
                // prints therminol temperature 

                // print outlet temperature 
                dbg!(heater_top_head_exit_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

                // print surface temperature 
                dbg!(heater_surface_array_temp);

                //// print therminol temperature 
                //dbg!("Therminol Array Temp: ", therminol_array_temperature);

                //// print twisted tape temperature 
                //dbg!("twisted tape Temp: 
                //note: conduction occurs, so first node is hotter\n 
                //than the therminol fluid", twisted_tape_temperature);

                // print loop time 
                // dbg diagnostics probably not the cause of mem leaks
                //println!("{:?}",time_taken_for_calculation_loop.as_micros());
            }

            // make axial connections to BCs 
            //
            // note: need to speed up this part, too slow

            heater_bottom_head.pipe_fluid_array.link_to_back(
                &mut inlet_bc,
                generic_advection_interaction
            ).unwrap();

            heater_v1.pipe_fluid_array.link_to_back(
                &mut heater_bottom_head.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            heater_v1.pipe_fluid_array.link_to_front(
                &mut heater_top_head.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            
            if connect_static_mixer_10 {
                heater_top_head.pipe_fluid_array.link_to_front(
                    &mut static_mixer_mx_10_object.pipe_fluid_array,
                    generic_advection_interaction
                ).unwrap();

                static_mixer_mx_10_object.pipe_fluid_array.link_to_front(
                    &mut static_mixer_mx_10_pipe.pipe_fluid_array,
                    generic_advection_interaction
                ).unwrap();

                static_mixer_mx_10_pipe.pipe_fluid_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();

            } else {

                heater_top_head.pipe_fluid_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();
            }
            
            //// and axial connections for heater top and bottom heads 
            //// to support 
            ////
            //// parallelise this

            //heater_bottom_head_bare.steel_shell.link_to_back(
            //    &mut structural_support_heater_bottom_head,
            //    support_conductance_interaction
            //).unwrap();

            //heater_top_head_bare.steel_shell.link_to_front(
            //    &mut structural_support_heater_top_head,
            //    support_conductance_interaction
            //).unwrap();

            //// link the top and bottom head support to the environment 
            //// parallelise this
            //
            //plus potential memory leak here

            //structural_support_heater_bottom_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();
            //structural_support_heater_top_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();


            // make other connections
            //
            // this is the serial version
            //heater_v2_bare.lateral_and_miscellaneous_connections(
            //    mass_flowrate,
            //    heater_power
            //);
            let wait: bool = false;

            // parallel calc probably not the cause of memory leak
            if wait {

                let ten_millis = time::Duration::from_millis(10);

                thread::sleep(ten_millis);

            } else {
                let porous_media_side_steady_state_power = Power::ZERO;
                let heater_top_bottom_head_power = Power::ZERO;
                let prandtl_wall_correction_setting = true;
                // make other connections by spawning a new thread 
                // this is the parallel version
                heater_v1.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_power,
                        porous_media_side_steady_state_power).unwrap();

                heater_bottom_head.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_top_bottom_head_power,
                        heater_top_bottom_head_power).unwrap();

                heater_top_head.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_top_bottom_head_power,
                        heater_top_bottom_head_power).unwrap();


                static_mixer_mx_10_object.lateral_and_miscellaneous_connections_mx10(
                    mass_flowrate);

                static_mixer_mx_10_pipe.lateral_and_miscellaneous_connections_mx10(
                    mass_flowrate);


                //// calculate timestep (serial method)
                //heater_v2_bare.advance_timestep(
                //    timestep);

                // calculate timestep (thread spawn method, parallel) 


                heater_v1.advance_timestep(timestep);
                heater_top_head.advance_timestep(timestep);
                heater_bottom_head.advance_timestep(timestep);
                static_mixer_mx_10_pipe.advance_timestep(timestep);
                static_mixer_mx_10_object.advance_timestep(timestep);



            } 


            // for outlet temperature, we use static mixer mx10 pipe 
            // temperature 

            final_outlet_temp = 
                static_mixer_mx_10_pipe
                .pipe_fluid_array
                .try_get_bulk_temperature()
                .unwrap();

            simulation_time += timestep;

            let time_taken_for_calculation_loop = loop_time.elapsed().unwrap()
            - loop_time_start;

            dbg!(time_taken_for_calculation_loop);

        }
        // assert final temp 
        //
        // it's within 1.3 degc of expt data, not
        approx::assert_abs_diff_eq!(
            final_experimental_outlet_temp.get::<degree_celsius>(),
            final_outlet_temp.get::<degree_celsius>(),
            epsilon=0.2);

    });

    main_loop.join().unwrap();





    // once simulation completed, write data


    //todo!("haven't coded csv writing file")



}
