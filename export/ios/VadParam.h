#import <Foundation/Foundation.h>

@interface VadParams : NSObject

@property (nonatomic, assign) NSUInteger frameSize;
@property (nonatomic, assign) float threshold;
@property (nonatomic, assign) NSUInteger minSilenceDurationMs;
@property (nonatomic, assign) NSUInteger speechPadMs;
@property (nonatomic, assign) NSUInteger minSpeechDurationMs;
@property (nonatomic, assign) float maxSpeechDurationS;
@property (nonatomic, assign) NSUInteger sampleRate;

@end

@implementation VadParams

@end